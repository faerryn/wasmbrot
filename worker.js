"use strict";

onmessage = function(msg) {
  console.log("Worker not ready yet!");
};

import init, { Wasmbrot } from "./wasmbrot.js";

let memory = null;
let canvas = null;
let setup = false;
let ctx;
let wasmbrot;
let image;

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
    const stepSize = msg.data.stepSize;

    if (!setup) {
      canvas = msg.data.canvas;
      ctx = canvas.getContext("2d");
    }

    const width = canvas.width;
    const height = canvas.height;

    wasmbrot = Wasmbrot.bounds(width, height, left, top, pixelSize);
    const colorsPtr = wasmbrot.colors();
    const colors = new Uint8ClampedArray(
      memory.buffer,
      colorsPtr,
      4 * width * height
    );
    image = new ImageData(colors, width);

    if (!setup) {
      function draw() {
        wasmbrot.step(stepSize);
        wasmbrot.colorize();

        ctx.putImageData(image, 0, 0);

        setTimeout(function() {
          requestAnimationFrame(draw);
        }, wasmbrot.depth() / 128);
      }

      requestAnimationFrame(draw);
    }

    setup = true;
  }
};
