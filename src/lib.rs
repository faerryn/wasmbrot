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
    left: f32,
    top: f32,
    scale: f32,
    depths: Vec<i32>, // -1 is synonymous with STOP
    zs: Vec<(Complex, Complex)>,
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
            left,
            top,
            scale,
            depths: vec![0; width * height],
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
        }
    }

    pub fn tick(&mut self) {
        for idx in 0..self.depths.len() {
            let depth = &mut self.depths[idx];

            if *depth == -1 {
                return;
            }

            let (z, c) = &mut self.zs[idx];

            *z = *z * *z + *c;

            if z.abs_sqr() > 4.0 {
                *depth = -1;
                return;
            }

            *depth += 1;
        }
    }

    pub fn depths(&self) -> *const i32 {
        self.depths.as_ptr()
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
