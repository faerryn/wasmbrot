mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Wasmbrot {
    width: usize,
    height: usize,
    depth: u32,
    depths: Vec<(bool, u32)>, // -1 is synonymous with STOP
    zs: Vec<(Complex, Complex)>,
    colors: Vec<u8>,
}

#[wasm_bindgen]
impl Wasmbrot {
    pub fn new(width: usize, height: usize, center_x: f32, center_y: f32, scale: f32) -> Wasmbrot {
        let scale = scale * 2.0 / width.min(height) as f32;
        let left = center_x - scale * width as f32 / 2.0;
        let top = center_y + scale * height as f32 / 2.0;

        Wasmbrot {
            width,
            height,
            depth: 0,
            depths: vec![(true, 0); width * height],
            zs: (0..width * height)
                .map(|idx| {
                    let row = idx / width;
                    let col = idx % width;

                    let x = left + col as f32 * scale;
                    let y = top - row as f32 * scale;

                    let point = Complex::new(x, y);

                    (point, point)
                })
                .collect(),
            colors: vec![0; 4 * width * height],
        }
    }

    pub fn tick(&mut self) {
        self.depth += 1;

        for idx in 0..(self.width * self.height) {
            let (in_set, point_depth) = &mut self.depths[idx];

            if *in_set {
                let (z, c) = &mut self.zs[idx];

                *z = *z * *z + *c;

                if z.abs_sqr() > 4.0 {
                    *in_set = false;
                } else {
                    *point_depth += 1;
                }
            }
        }
    }

    pub fn colorize(&mut self) {
        for idx in 0..(self.width * self.height) {
            let (_, point_depth) = &self.depths[idx];

            let grey = (*point_depth as f32 / self.depth as f32).powf(1.0);

            self.colors[4 * idx] = (grey * 255.0) as u8;
            self.colors[4 * idx + 1] = (grey * 255.0) as u8;
            self.colors[4 * idx + 2] = (grey * 255.0) as u8;
            self.colors[4 * idx + 3] = 255;
        }
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn colors(&self) -> *const u8 {
        self.colors.as_ptr()
    }
}

#[derive(Clone, Copy)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    fn new(real: f32, imag: f32) -> Complex {
        Complex { real, imag }
    }

    fn abs_sqr(&self) -> f32 {
        self.real * self.real + self.imag * self.imag
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// impl std::ops::AddAssign for Complex {
//     fn add_assign(&mut self, rhs: Complex) {
//         self.real += rhs.real;
//         self.imag += rhs.imag;
//     }
// }
//
// impl std::ops::Sub for Complex {
//     type Output = Complex;
//
//     fn sub(self, rhs: Complex) -> Self::Output {
//         Complex {
//             real: self.real - rhs.real,
//             imag: self.imag - rhs.imag,
//         }
//     }
// }
//
// impl std::ops::SubAssign for Complex {
//     fn sub_assign(&mut self, rhs: Complex) {
//         self.real -= rhs.real;
//         self.imag -= rhs.imag;
//     }
// }

impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

// impl std::ops::MulAssign for Complex {
//     fn mul_assign(&mut self, rhs: Complex) {
//         let real = self.real * rhs.real - self.imag * rhs.imag;
//         let imag = self.real * rhs.imag + self.imag * rhs.real;
//         self.real = real;
//         self.imag = imag;
//     }
// }
//
// impl std::ops::Div for Complex {
//     type Output = Complex;
//
//     fn div(self, rhs: Complex) -> Self::Output {
//         let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
//         Complex {
//             real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
//             imag: (self.imag * rhs.real - self.real * rhs.imag) / denom,
//         }
//     }
// }
//
// impl std::ops::DivAssign for Complex {
//     fn div_assign(&mut self, rhs: Complex) {
//         let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
//         let real = (self.real * rhs.real + self.imag * rhs.imag) / denom;
//         let imag = (self.imag * rhs.real - self.real * rhs.imag) / denom;
//         self.real = real;
//         self.imag = imag;
//     }
// }
