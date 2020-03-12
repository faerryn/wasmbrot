use num::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasmbrot {
    multi: f64,
    burning: bool,
    julia_re: Option<f64>,
    julia_im: Option<f64>,
    escape: f64,
    width: usize,
    height: usize,
    depth: u32,
    depths: Vec<u32>,
    pixel_states: Vec<PixelState>,
    zs: Vec<Complex<f64>>,
    cs: Vec<Complex<f64>>,
    colors: Vec<u8>,
}

#[wasm_bindgen]
impl Wasmbrot {
    pub fn new(
        multi: f64,
        burning: bool,
        julia_re: Option<f64>,
        julia_im: Option<f64>,
        escape: f64,
        width: usize,
        height: usize,
        left: f64,
        top: f64,
        pixel_width: f64,
        pixel_height: f64,
    ) -> Wasmbrot {
        let mut zs = Vec::with_capacity(width * height);
        let mut cs = Vec::with_capacity(width * height);
        let mut colors = Vec::with_capacity(4 * width * height);

        for idx in 0..width * height {
            let row = idx / width;
            let col = idx % width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            zs.push(Complex::new(x, y));

            cs.push(Complex::new(julia_re.unwrap_or(x), julia_im.unwrap_or(y)));

            colors.push(0);
            colors.push(0);
            colors.push(0);
            colors.push(0xff);
        }

        Wasmbrot {
            multi,
            burning,
            julia_re,
            julia_im,
            escape,
            width,
            height,
            depth: 0,
            depths: vec![0; width * height],
            pixel_states: vec![PixelState::InSet; width * height],
            zs,
            cs,
            colors,
        }
    }

    pub fn reparam(
        &mut self,
        multi: f64,
        burning: bool,
        julia_re: Option<f64>,
        julia_im: Option<f64>,
        escape: f64,
        left: f64,
        top: f64,
        pixel_width: f64,
        pixel_height: f64,
    ) {
        self.multi = multi;
        self.burning = burning;
        self.julia_re = julia_re;
        self.julia_im = julia_im;
        self.escape = escape;

        self.depth = 0;
        for idx in 0..self.width * self.height {
            self.depths[idx] = 0;

            self.pixel_states[idx] = PixelState::InSet;

            self.colors[4 * idx] = 0;
            self.colors[4 * idx + 1] = 0;
            self.colors[4 * idx + 2] = 0;
            self.colors[4 * idx + 3] = 0xff;

            let row = idx / self.width;
            let col = idx % self.width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            self.zs[idx] = Complex::new(x, y);

            self.cs[idx] = Complex::new(self.julia_re.unwrap_or(x), self.julia_im.unwrap_or(y));
        }
    }

    pub fn step(&mut self, step_size: u32) -> bool {
        self.depth += step_size;

        let mut changed = false;

        let multi_style = if self.multi == 2.0 {
            MultiStyle::Square
        } else if self.multi == self.multi.trunc() {
            MultiStyle::Uint(self.multi as u32)
        } else {
            MultiStyle::Float(self.multi)
        };

        'pixels: for idx in 0..(self.width * self.height) {
            if self.pixel_states[idx] == PixelState::InSet {
                changed = true;

                let c = &self.cs[idx];
                let z = &mut self.zs[idx];

                for _ in 0..step_size {
                    if self.burning {
                        z.re = z.re.abs();
                        z.im = z.im.abs();
                    }

                    if z.norm_sqr() > self.escape * self.escape {
                        self.pixel_states[idx] = PixelState::NotRendered;
                        continue 'pixels;
                    }

                    self.depths[idx] += 1;

                    match multi_style {
                        MultiStyle::Square => *z = *z * *z + c,
                        MultiStyle::Uint(multi) => *z = z.powu(multi) + c,
                        MultiStyle::Float(multi) => *z = z.powf(multi) + c,
                    }
                }
            }
        }

        changed
    }

    pub fn colorize(&mut self, color_dist: f64) {
        for idx in 0..self.width * self.height {
            if self.pixel_states[idx] == PixelState::NotRendered {
                self.pixel_states[idx] = PixelState::Rendered;

                let depth = self.depths[idx] as f64 / color_dist;

                let r = depth.sin();
                let r = r * r;
                let r = (r * 255.0) as u8;

                let g = (depth + std::f64::consts::FRAC_PI_4).sin();
                let g = g * g;
                let g = (g * 255.0) as u8;

                let b = (depth + std::f64::consts::FRAC_PI_2).sin();
                let b = b * b;
                let b = (b * 255.0) as u8;

                self.colors[4 * idx] = r;
                self.colors[4 * idx + 1] = g;
                self.colors[4 * idx + 2] = b;
                self.colors[4 * idx + 3] = 0xff;
            }
        }
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn colors(&self) -> *const u8 {
        self.colors.as_ptr()
    }
}

enum MultiStyle {
    Square,
    Uint(u32),
    Float(f64),
}

#[derive(Clone, PartialEq)]
enum PixelState {
    InSet,
    NotRendered,
    Rendered,
}
