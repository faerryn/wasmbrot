use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasmbrot {
    width: usize,
    height: usize,
    left: f64,
    right: f64,
    top: f64,
    down: f64,
    pixel_size: f64,
    depth: u32,
    depths: Vec<u32>,
    in_set: Vec<bool>,
    zs: Vec<Complex>,
    cs: Vec<Complex>,
    colors: Vec<u8>,
}

#[wasm_bindgen]
impl Wasmbrot {
    pub fn bounds(width: usize, height: usize, left: f64, top: f64, pixel_size: f64) -> Wasmbrot {
        Wasmbrot {
            width,
            height,
            left,
            right: left + width as f64 * pixel_size,
            top,
            down: top - height as f64 * pixel_size,
            pixel_size,
            depth: 0,
            depths: vec![0; width * height],
            in_set: vec![true; width * height],
            zs: vec![Complex::new(0.0, 0.0); width * height],
            cs: (0..width * height)
                .map(|idx| {
                    let row = idx / width;
                    let col = idx % width;

                    let x = left + col as f64 * pixel_size;
                    let y = top - row as f64 * pixel_size;

                    Complex::new(x, y)
                })
                .collect(),
            colors: vec![0; 4 * width * height],
        }
    }

    pub fn recycle(
        width: usize,
        height: usize,
        left: f64,
        top: f64,
        pixel_size: f64,
        mut old: Wasmbrot,
    ) -> Wasmbrot {
        old.depths.clear();
        old.depths.resize(width * height, 0);
        old.in_set.clear();
        old.in_set.resize(width * height, true);
        old.zs.clear();
        old.zs.resize(width * height, Complex::new(0.0, 0.0));
        old.colors.resize(4 * width * height, 0);

        old.cs.clear();
        old.cs.reserve(width * height - old.cs.len());

        for idx in 0..width * height {
            let row = idx / width;
            let col = idx % width;

            let x = left + col as f64 * pixel_size;
            let y = top - row as f64 * pixel_size;

            old.cs.push(Complex::new(x, y));
        }

        Wasmbrot {
            width,
            height,
            left,
            right: left + width as f64 * pixel_size,
            top,
            down: top - height as f64 * pixel_size,
            pixel_size,
            depth: 0,
            depths: old.depths,
            in_set: old.in_set,
            zs: old.zs,
            cs: old.cs,
            colors: old.colors,
        }
    }

    pub fn step(&mut self, step_size: u32) -> bool {
        self.depth += step_size;
        let mut changed = false;

        'pixels: for idx in 0..(self.width * self.height) {
            let in_set = &mut self.in_set[idx];

            if *in_set {
                changed = true;

                let z = &mut self.zs[idx];

                for partial_step in 0..step_size {
                    if z.modulus_squared() > 4.0 {
                        self.depths[idx] += partial_step;
                        *in_set = false;
                        continue 'pixels;
                    }

                    z.mut_square_add(&self.cs[idx]);
                }

                self.depths[idx] += step_size;
            }
        }

        changed
    }

    pub fn colorize(&mut self) {
        for idx in 0..self.width * self.height {
            let (r, g, b) = if self.in_set[idx] {
                (0, 0, 0)
            } else {
                let depth = self.depths[idx] as f64 / 10.0;

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

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn down(&self) -> f64 {
        self.down
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }
}

// fn hsl_to_rgb(hue: f64, saturation: f64, luminance: f64) -> (f64, f64, f64) {
//     let hue = hue % 360.0;
//     let chroma = (1.0 - (2.0 * luminance - 1.0).abs()) * saturation;
//     let cube_face = hue / 60.0;
//     let secondary_chroma = chroma * (1.0 - (cube_face % 2.0 - 1.0).abs());
//     let m = luminance - chroma / 2.0;
//
//     match cube_face as u32 {
//         0 => (chroma + m, secondary_chroma + m, m),
//         1 => (secondary_chroma + m, chroma + m, m),
//         2 => (m, chroma + m, secondary_chroma + m),
//         3 => (m, secondary_chroma + m, chroma + m),
//         4 => (secondary_chroma + m, m, chroma + m),
//         5 => (chroma + m, m, secondary_chroma + m),
//         _ => (0.0, 0.0, 0.0),
//     }
// }

#[derive(Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    #[inline(always)]
    fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    #[inline(always)]
    fn modulus_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    #[inline(always)]
    fn mut_square_add(&mut self, c: &Complex) {
        let real = self.real * self.real - self.imag * self.imag + c.real;
        let imag = 2.0 * (self.real * self.imag) + c.imag;
        self.real = real;
        self.imag = imag;
    }
}
