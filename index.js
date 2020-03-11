"use strict";

const workerLen = window.navigator.hardwareConcurrency;

const stepSize = 1;
const scale = 2.0;
const center_x = 0.0;
const center_y = 0.0;

const width = window.innerWidth;
const height = window.innerHeight;

const pixelSize = (scale * 2) / Math.min(width, height);

const left = center_x - (pixelSize * width) / 2;
const right = center_x + (width * pixelSize) / 2;

const top = center_y + (pixelSize * height) / 2;
const down = center_y - (height * pixelSize) / 2;

for (let i = 0; i < workerLen; i += 1) {
  const canvas = document.createElement("canvas");
  document.body.appendChild(canvas);

  const canvasHeight =
    Math.round((height / workerLen) * (i + 1)) -
    Math.round((height / workerLen) * i);

  console.log(canvasHeight);

  canvas.width = width;
  canvas.height = canvasHeight;

  const offscreen = canvas.transferControlToOffscreen();

  const workerTop = top - Math.round((height / workerLen) * i) * pixelSize;

  const worker = new Worker("worker.js", { type: "module" });
  worker.onmessage = function(msg) {
    if (msg.data === "Ready") {
      worker.postMessage(
        {
          canvas: offscreen,
          left,
          top: workerTop,
          pixelSize,
          stepSize
        },
        [offscreen]
      );
    }
  };
}
