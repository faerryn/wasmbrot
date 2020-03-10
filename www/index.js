"use strict";

import { Wasmbrot } from "wasmbrot";
import { memory } from "wasmbrot/wasmbrot_bg";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const width = (canvas.width = window.innerWidth);
const height = (canvas.height = window.innerHeight);

const colors = new Uint8ClampedArray(4 * width * height);
let maxDepth = 0;

const wasmbrot = Wasmbrot.new(width, height, 0.0, 0.0, 1.0);
const depthsPtr = wasmbrot.depths();

function draw() {
  const depths = new Uint32Array(memory.buffer, depthsPtr, width * height);

  for (let idx = 0; idx < width * height; idx += 1) {
    const gray = Math.sqrt(maxDepth - depths[idx]);
    depths[4 * idx] = gray;
    depths[4 * idx + 1] = gray;
    depths[4 * idx + 2] = gray;
    depths[4 * idx + 3] = 255;
  }

  const image = new ImageData(colors, width);
  ctx.putImageData(image, 0, 0);

  window.requestAnimationFrame(draw);
}

window.requestAnimationFrame(draw);

while (true) {
  wasmbrot.tick();
  depths += 1;
}
