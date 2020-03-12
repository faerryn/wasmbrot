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
let colorDist;

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
    const pixelWidth = msg.data.pixelWidth;
    const pixelHeight = msg.data.pixelHeight;
    const multi = msg.data.multi;
    const burning = msg.data.burning;
    const juliaRe = msg.data.juliaRe;
    const juliaIm = msg.data.juliaIm;
    const escape = msg.data.escape;

    maxDepth = msg.data.maxDepth;
    stepSize = msg.data.stepSize;
    colorDist = msg.data.colorDist;

    if (!alreadySetup) {
      canvas = msg.data.canvas;
      ctx = canvas.getContext("2d", { alpha: false });
      width = canvas.width;
      height = canvas.height;
      wasmbrot = Wasmbrot.new(
        multi,
        burning,
        juliaRe,
        juliaIm,
        escape,
        width,
        height,
        left,
        top,
        pixelWidth,
        pixelHeight
      );

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
      wasmbrot.reparam(
        multi,
        burning,
        juliaRe,
        juliaIm,
        escape,
        left,
        top,
        pixelWidth,
        pixelHeight
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
  const changed = wasmbrot.step(stepSize); // only evaluate the step function if not stopped
  if (changed) {
    wasmbrot.colorize(colorDist);

    ctx.putImageData(image, 0, 0);

    if (wasmbrot.depth() < maxDepth) {
      setTimeout(function() {
        requestAnimationFrame(draw);
      }, 1000 / 60);
      return;
    }
  }

  // reaching here should stop
  stopped = true;
}
