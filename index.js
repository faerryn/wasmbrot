"use strict";

const width = window.innerWidth;
const height = window.innerHeight;

let colorDist;
let view;
let stepSize;

let canvasRows;
let canvasCols;
let zoom = 5;

let maxDwell = Infinity;

let canvases = [];
let workers = [];

let workersReceivedCanvas = false;

let parameters = new Parameters();

(window.onpopstate = function() {
    const params         = new URL(document.location).searchParams;
    parameters.colorDist = parseFloat(params.get("colorDist")) || 16;
    parameters.x         = parseFloat(params.get("x")) || 0;
    parameters.y         = parseFloat(params.get("y")) || 0;
    parameters.scale     = parseFloat(params.get("scale")) || 2;

    view                 = new View(parameters);

    try {
        stepSize           = BigInt(params.get("stepSize"));
    } catch (e) {
        stepSize           = 16n;
    }

    canvasRows           =
        parseInt(params.get("canvasRows")) ||
        Math.ceil(Math.sqrt(navigator.hardwareConcurrency));

    canvasCols           =
        parseInt(params.get("canvasCols")) ||
        Math.ceil(navigator.hardwareConcurrency / canvasRows);

    zoom                 = parseFloat(params.get("zoom")) || 4;

    maxDwell             = parseFloat(params.get("maxDwell")) || Infinity;

    if (workersReceivedCanvas) {
        reparam();
    }
})(); // run this function now!

const workerLen = canvasRows * canvasCols;
let notReady = workerLen;
for (let row = 0; row < canvasRows; row += 1) {
    for (let col = 0; col < canvasCols; col += 1) {
        const canvas = document.createElement("canvas");
        canvas.style.zIndex = 0;
        document.body.appendChild(canvas);

        const left = Math.round((width / canvasCols) * col);
        const top = Math.round((height / canvasRows) * row);
        canvas.style = `left: ${left}px; top: ${top}px;`;

        const canvasWidth =
              Math.round((width / canvasCols) * (col + 1)) -
              Math.round((width / canvasCols) * col);

        const canvasHeight =
              Math.round((height / canvasRows) * (row + 1)) -
              Math.round((height / canvasRows) * row);

        canvas.width = canvasWidth;
        canvas.height = canvasHeight;

        const offscreen = canvas.transferControlToOffscreen();
        canvases.push(offscreen);

        const worker = new Worker(`worker.js?random=${Math.random()}`, {
            type: "module"
        });
        workers.push(worker);

        worker.onmessage = function(msg) {
            notReady -= 1;
            if (notReady === 0) {
                // all workers ready, reparam!
                reparam();
            }
        };
    }
}

function reparam() {
    for (let i = 0; i < workerLen; i += 1) {
        const canvas = canvases[i];
        const worker = workers[i];

        const row = Math.floor(i / canvasCols);
        const col = i % canvasCols;

        const left =
              view.left + Math.round((width / canvasCols) * col) * view.pixelSize;
        const top =
              view.top - Math.round((height / canvasRows) * row) * view.pixelSize;

        worker.postMessage(
            {
                canvas: !workersReceivedCanvas ? canvas : undefined,
                left,
                top,
                pixelWidth: view.pixelSize,
                pixelHeight: view.pixelSize,
                stepSize,
                maxDwell,
                parameters
            },
            !workersReceivedCanvas ? [canvas] : []
        );
    }

    workersReceivedCanvas = true;
}

const overlayWidth = width / zoom;
const overlayHeight = height / zoom;

window.onclick = function(e) {
    if (notReady > 0) {
        console.log(`${notReady} workers at not ready yet`);
    }

    const row = clamp(e.clientX, overlayWidth / 2, width - overlayWidth / 2);
    const col = clamp(e.clientY, overlayHeight / 2, height - overlayHeight / 2);

    const x = view.left + row * view.pixelSize;
    const y = view.top - col * view.pixelSize;

    parameters.x = x;
    parameters.y = y;
    parameters.scale = view.scale / zoom;

    view = new View(parameters);

    window.history.pushState(
        "",
        "",
        `?${new URLSearchParams(parameters).toString()}`
    );

    reparam();
};

let vanishPreview;
const banner = document.getElementById("banner");
banner.width = Math.min(overlayWidth, overlayHeight) / 2;
banner.height = banner.width;

window.onmousemove = function(e) {
    if (vanishPreview !== undefined) {
        clearTimeout(vanishPreview);
    }

    const row = clamp(e.x, overlayWidth / 2, width - overlayWidth / 2);
    const col = clamp(e.y, overlayHeight / 2, height - overlayHeight / 2);

    const left = row - overlayWidth / 2;
    const top = col - overlayHeight / 2;

    banner.hidden = false;

    vanishPreview = setTimeout(function() {
        banner.hidden = true;
    }, 2000);
};

function clamp(x, min, max) {
    return Math.min(Math.max(x, min), max);
}

function Parameters(
    x,
    y,
    scale,
    colorDist
) {
    this.x = x || 0;
    this.y = y || 0;
    this.scale = scale || 2;
    this.colorDist = colorDist || 16;
}

function View(params) {
    this.x = params.x;
    this.y = params.y;
    this.scale = params.scale;

    this.pixelSize = (params.scale * 2) / Math.min(width, height);

    this.left = this.x - (this.pixelSize * width) / 2;
    this.top = this.y + (this.pixelSize * height) / 2;
}
