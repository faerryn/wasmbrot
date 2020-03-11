"use strict";

const width = window.innerWidth;
const height = window.innerHeight;

const zoom = 5;

const overlay = document.getElementById("overlay");
overlay.width = width;
overlay.height = height;
const overlayCtx = overlay.getContext("2d");

const overlayWidth = width / zoom;
const overlayHeight = height / zoom;

const workerLen = Math.max(window.navigator.hardwareConcurrency - 1, 1);

const stepSize = 32;

function View(x, y, scale) {
  this.x = x;
  this.y = y;
  this.scale = scale;

  this.pixelSize = (scale * 2) / Math.min(width, height);

  this.left = x - (this.pixelSize * width) / 2;
  this.top = y + (this.pixelSize * height) / 2;
}

let view = new View(0.0, 0.0, 2.0);

let canvases = [];
let workers = [];
let notReady = workerLen;

for (let i = 0; i < workerLen; i += 1) {
  const canvas = document.createElement("canvas");
  document.body.appendChild(canvas);

  const canvasHeight =
    Math.round((height / workerLen) * (i + 1)) -
    Math.round((height / workerLen) * i);

  canvas.width = width;
  canvas.height = canvasHeight;

  const offscreen = canvas.transferControlToOffscreen();
  canvases.push(offscreen);

  const worker = new Worker("worker.js", { type: "module" });
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

function setup() {
  for (let i = 0; i < workerLen; i += 1) {
    const canvas = canvases[i];
    const worker = workers[i];

    worker.postMessage(
      {
        canvas,
        left: view.left,
        top: view.top - Math.round((height / workerLen) * i) * view.pixelSize,
        pixelSize: view.pixelSize,
        stepSize
      },
      [canvas]
    );
  }
}

function reset() {
  for (let i = 0; i < workerLen; i += 1) {
    const canvas = canvases[i];
    const worker = workers[i];

    worker.postMessage("Stop");

    worker.postMessage({
      left: view.left,
      top: view.top - Math.round((height / workerLen) * i) * view.pixelSize,
      pixelSize: view.pixelSize,
      stepSize
    });
  }
}

window.onclick = function(e) {
  console.log(e);

  if (notReady > 0) {
    console.log(`${notReady} workers at not ready yet`);
  }

  const row = clamp(e.clientX, overlayWidth / 2, width - overlayWidth / 2);
  const col = clamp(e.clientY, overlayHeight / 2, height - overlayHeight / 2);

  const x = view.left + row * view.pixelSize;
  const y = view.top - col * view.pixelSize;
  view = new View(x, y, view.scale / zoom);

  console.log(view);

  // send a message to the workers to reset
  reset();
};

let mouseX = 0;
let mouseY = 0;

window.onmousemove = function(e) {
  mouseX = e.clientX;
  mouseY = e.clientY;
  overlayCtx.clearRect(0, 0, width, height);

  const row = clamp(mouseX, overlayWidth / 2, width - overlayWidth / 2);
  const col = clamp(mouseY, overlayHeight / 2, height - overlayHeight / 2);

  const left = row - overlayWidth / 2;
  const top = col - overlayHeight / 2;

  overlayCtx.strokeStyle = "white";

  overlayCtx.strokeRect(left, top, overlayWidth, overlayHeight);
};

function clamp(x, min, max) {
  return Math.min(Math.max(x, min), max);
}
