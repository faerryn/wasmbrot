"use strict";

import { Wasmbrot } from "wasmbrot";
import { memory } from "wasmbrot/wasmbrot_bg";

const ctx = canvas.getContext("2d");

const width = (canvas.width = window.innerWidth);
const height = (canvas.height = window.innerHeight);

const maxDepth = 20;

const wasmbrot = Wasmbrot.new(width, height, 0.0, 0.0, Math.sqrt(2.0));

const colorsPtr = wasmbrot.colors();
const colors = new Uint8ClampedArray(
  memory.buffer,
  colorsPtr,
  4 * width * height
);
const image = new ImageData(colors, width);

function draw() {
  wasmbrot.colorize();
  ctx.putImageData(image, 0, 0);

  wasmbrot.tick();

  if (wasmbrot.depth() < maxDepth) {
    window.requestAnimationFrame(draw);
  }
}

window.requestAnimationFrame(draw);
