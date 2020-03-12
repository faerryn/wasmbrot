"use strict";
const width = window.innerWidth;
const height = window.innerHeight;

let multi;
let burning;
let juliaRe;
let juliaIm;
let escape;
let colorDist;
let view;
let alreadySetup = false;

(window.onpopstate = function() {
  const params = new URL(document.location).searchParams;

  multi = parseFloat(params.get("multi"));
  if (isNaN(multi)) {
    multi = 2;
  }

  burning = (parseInt(params.get("burning")) === 1) | false;

  juliaRe = parseFloat(params.get("juliaRe"));

  juliaIm = parseFloat(params.get("juliaIm"));

  escape = parseFloat(params.get("escape"));
  if (isNaN(escape)) {
    escape = 2;
  }

  colorDist = parseFloat(params.get("colorDist"));
  if (isNaN(colorDist)) {
    colorDist = 10;
  }

  let x = parseFloat(params.get("x"));
  if (isNaN(x)) {
    x = 0;
  }

  let y = parseFloat(params.get("y"));
  if (isNaN(y)) {
    y = 0;
  }

  let scale = parseFloat(params.get("scale"));
  if (isNaN(scale)) {
    scale = 2;
  }

  view = new View(x, y, scale);

  if (alreadySetup) {
    setup();
  }
})();

function currentState() {
  return {
    multi,
    burning,
    juliaRe,
    juliaIm,
    escape,
    x: view.x,
    y: view.y,
    scale: view.scale,
    colorDist
  };
}

function View(x, y, scale) {
  this.x = x;
  this.y = y;
  this.scale = scale;

  this.pixelSize = (scale * 2) / Math.min(width, height);

  this.left = x - (this.pixelSize * width) / 2;
  this.top = y + (this.pixelSize * height) / 2;
}

const zoom = 5;

const overlay = document.getElementById("overlay");
overlay.width = width;
overlay.height = height;
const overlayCtx = overlay.getContext("2d");

const overlayWidth = width / zoom;
const overlayHeight = height / zoom;

const canvasRows = Math.floor(height / 400);
const canvasCols = Math.floor(width / 400);
const workerLen = canvasRows * canvasCols;

const stepSize = 32;

const maxDepth = Infinity;

let canvases = [];
let workers = [];
let notReady = workerLen;

for (let row = 0; row < canvasRows; row += 1) {
  for (let col = 0; col < canvasCols; col += 1) {
    const canvas = document.createElement("canvas");
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
      if (msg.data === "Ready") {
        notReady -= 1;
        if (notReady === 0) {
          setup();
        }
      }
    };
  }
}

function setup() {
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
        canvas: !alreadySetup ? canvas : undefined,
        multi,
        burning,
        juliaRe: isNaN(juliaRe) ? null : juliaRe,
        juliaIm: isNaN(juliaIm) ? null : juliaIm,
        escape,
        left,
        top,
        pixelWidth: view.pixelSize,
        pixelHeight: view.pixelSize,
        stepSize,
        maxDepth,
        colorDist
      },
      !alreadySetup ? [canvas] : []
    );
  }

  alreadySetup = true;
}

overlay.onclick = function(e) {
  if (notReady > 0) {
    console.log(`${notReady} workers at not ready yet`);
  }

  const row = clamp(e.clientX, overlayWidth / 2, width - overlayWidth / 2);
  const col = clamp(e.clientY, overlayHeight / 2, height - overlayHeight / 2);

  const x = view.left + row * view.pixelSize;
  const y = view.top - col * view.pixelSize;
  view = new View(x, y, view.scale / zoom);

  window.history.pushState(
    "",
    "",
    `?${new URLSearchParams(currentState()).toString()}`
  );

  setup();
};

let vanishPreview;

overlay.onmousemove = function(e) {
  if (vanishPreview !== undefined) {
    clearTimeout(vanishPreview);
  }

  overlayCtx.clearRect(0, 0, width, height);

  const row = clamp(e.x, overlayWidth / 2, width - overlayWidth / 2);
  const col = clamp(e.y, overlayHeight / 2, height - overlayHeight / 2);

  const left = row - overlayWidth / 2;
  const top = col - overlayHeight / 2;

  overlayCtx.strokeStyle = "white";

  overlayCtx.strokeRect(left, top, overlayWidth, overlayHeight);

  vanishPreview = setTimeout(function() {
    overlayCtx.clearRect(0, 0, width, height);
  }, 2000);
};

function clamp(x, min, max) {
  return Math.min(Math.max(x, min), max);
}
