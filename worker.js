"use strict";

import init, { Wasmbrot } from "./wasmbrot.js";

let memory = null;
let ctx = null;
let wasmbrot;
let image;
let stepSize;
let maxDwell;
let colorDist;

async function run() {
  const wasm = await init();
  memory = wasm.memory;
  postMessage([]);
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

    maxDwell = msg.data.maxDwell;
    stepSize = msg.data.stepSize;
    colorDist = msg.data.colorDist;

    if (ctx === null) {
      const canvas = msg.data.canvas;
      ctx = canvas.getContext("2d", { alpha: false });
      const width = canvas.width;
      const height = canvas.height;
      wasmbrot = Wasmbrot.new(
        multi,
        burning === 1,
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
    } else {
      wasmbrot.reparam(
        multi,
        burning === 1,
        juliaRe,
        juliaIm,
        escape,
        left,
        top,
        pixelWidth,
        pixelHeight
      );
    }

    ctx.putImageData(image, 0, 0);
    requestAnimationFrame(draw);
  }
};

function draw() {
  const result = wasmbrot.step(stepSize);

  if (result.new_colors) {
    wasmbrot.colorize(colorDist);

    ctx.putImageData(image, 0, 0);
  }

  if (!result.all_known && wasmbrot.dwell() < maxDwell) {
    requestAnimationFrame(draw);
  }
}
