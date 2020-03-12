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
    in_set: Vec<bool>,
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

        for idx in 0..width * height {
            let row = idx / width;
            let col = idx % width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            let z = Complex::new(x, y);

            zs.push(z);

            let c = Complex::new(julia_re.unwrap_or(x), julia_im.unwrap_or(y));

            cs.push(c);
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
            in_set: vec![true; width * height],
            zs,
            cs,
            colors: vec![0; 4 * width * height],
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
            self.in_set[idx] = true;
            self.colors[idx] = 0;
            self.colors[idx + 1] = 0;
            self.colors[idx + 2] = 0;
            self.colors[idx + 3] = 0xff;

            let row = idx / self.width;
            let col = idx % self.width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            let z = Complex::new(x, y);

            self.zs[idx] = z;

            let c = Complex::new(self.julia_re.unwrap_or(x), self.julia_im.unwrap_or(y));

            self.cs[idx] = c;
        }
    }

    pub fn step(&mut self, step_size: u32) -> bool {
        self.depth += step_size;

        let mut changed = false;

        'pixels: for idx in 0..(self.width * self.height) {
            let in_set = &mut self.in_set[idx];

            if *in_set {
                changed = true;

                let c = &self.cs[idx];
                let z = &mut self.zs[idx];

                for _ in 0..step_size {
                    if z.norm_sqr() > self.escape * self.escape {
                        *in_set = false;
                        continue 'pixels;
                    }

                    self.depths[idx] += 1;

                    if self.burning {
                        z.re = z.re.abs();
                        z.im = z.im.abs();
                    }

                    if self.multi == self.multi.trunc() {
                        *z = z.powu(self.multi as u32) + c;
                    } else {
                        *z = z.powf(self.multi) + c;
                    }
                }
            }
        }

        changed
    }

    pub fn colorize(&mut self, color_dist: f64) {
        for idx in 0..self.width * self.height {
            let (r, g, b) = if self.in_set[idx] {
                (0, 0, 0)
            } else {
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

                (r, g, b)
            };

            self.colors[4 * idx] = r;
            self.colors[4 * idx + 1] = g;
            self.colors[4 * idx + 2] = b;
            self.colors[4 * idx + 3] = 0xff;
        }
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn colors(&self) -> *const u8 {
        self.colors.as_ptr()
    }
}
