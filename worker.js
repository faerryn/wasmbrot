"use strict";

onmessage = function(msg) {
  console.log("Worker not ready yet!");
};

import init, { Wasmbrot } from "./wasmbrot.js";

let memory;

async function run() {
  const wasm = await init();
  memory = wasm.memory;
  postMessage("Ready");
}

run();

onmessage = function(msg) {
  const canvas = msg.data.canvas;
  const left = msg.data.left;
  const top = msg.data.top;
  const pixelSize = msg.data.pixelSize;
  const stepSize = msg.data.stepSize;

  const width = canvas.width;
  const height = canvas.height;
  const ctx = canvas.getContext("2d");

  const wasmbrot = Wasmbrot.bounds(width, height, left, top, pixelSize);

  const colorsPtr = wasmbrot.colors();
  const colors = new Uint8ClampedArray(
    memory.buffer,
    colorsPtr,
    4 * width * height
  );
  const image = new ImageData(colors, width);

  function draw() {
    wasmbrot.step(stepSize);
    wasmbrot.colorize();

    ctx.putImageData(image, 0, 0);
    requestAnimationFrame(draw);
  }

  requestAnimationFrame(draw);
};
