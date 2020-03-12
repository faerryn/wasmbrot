"use strict";

onmessage = function(msg) {
  console.log("Worker not ready yet!");
};

import init, { Wasmbrot } from "./wasmbrot.js";

let memory = null;
let canvas = null;
let alreadySetup = false;
let ctx;
let wasmbrot;
let image;
let width;
let height;
let stopped;
let stepSize;
let maxDepth;

async function run() {
  const wasm = await init();
  memory = wasm.memory;
  postMessage("Ready");
}

run();

onmessage = function(msg) {
  if (memory === null) {
    console.log("Worker not ready yet!");
  } else {
    const left = msg.data.left;
    const top = msg.data.top;
    const pixelSize = msg.data.pixelSize;

    maxDepth = msg.data.maxDepth;
    stepSize = msg.data.stepSize;

    if (!alreadySetup) {
      canvas = msg.data.canvas;
      ctx = canvas.getContext("2d");
      width = canvas.width;
      height = canvas.height;
      wasmbrot = Wasmbrot.bounds(width, height, left, top, pixelSize);

      const colorsPtr = wasmbrot.colors();
      const colors = new Uint8ClampedArray(
        memory.buffer,
        colorsPtr,
        4 * width * height
      );
      image = new ImageData(colors, width);

      stopped = false;
      requestAnimationFrame(draw);
      alreadySetup = true;
    } else {
      wasmbrot = Wasmbrot.recycle(
        width,
        height,
        left,
        top,
        pixelSize,
        wasmbrot
      );

      const colorsPtr = wasmbrot.colors();
      const colors = new Uint8ClampedArray(
        memory.buffer,
        colorsPtr,
        4 * width * height
      );
      image = new ImageData(colors, width);

      if (stopped) {
        stopped = false;
        requestAnimationFrame(draw);
      }
    }
  }
};

function draw() {
  wasmbrot.step(stepSize); // only evaluate the step function if not stopped
  wasmbrot.colorize();

  ctx.putImageData(image, 0, 0);

  if (wasmbrot.depth() < maxDepth) {
    requestAnimationFrame(draw);
  }
}
