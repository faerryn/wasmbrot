use num::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasmbrot {
    width: usize,
    height: usize,
    dwell: u64,
    points: Vec<Point>,
    zs: Vec<Complex<f64>>,
    cs: Vec<Complex<f64>>,
    period_checks: Vec<PeriodCheck>,
    colors: Vec<u8>,
}

#[wasm_bindgen]
impl Wasmbrot {
    pub fn new(
        width: usize,
        height: usize,
        left: f64,
        top: f64,
        pixel_width: f64,
        pixel_height: f64,
    ) -> Wasmbrot {
        let mut zs = Vec::with_capacity(width * height);
        let mut cs = Vec::with_capacity(width * height);
        let mut period_checks = Vec::with_capacity(width * height);
        let mut colors = Vec::with_capacity(4 * width * height);

        for idx in 0..width * height {
            let row = idx / width;
            let col = idx % width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            let z = Complex::new(x, y);
            zs.push(z);
            period_checks.push(PeriodCheck::new(z));
            cs.push(z);

            colors.push(0);
            colors.push(0);
            colors.push(0);
            colors.push(0xff);
        }

        Wasmbrot {
            width,
            height,
            dwell: 0,
            points: vec![Point::Unknown(0); width * height],
            zs,
            cs,
            period_checks,
            colors,
        }
    }

    pub fn reparam(&mut self, left: f64, top: f64, pixel_width: f64, pixel_height: f64) {
        self.dwell = 0;
        for idx in 0..self.width * self.height {
            self.points[idx] = Point::Unknown(0);

            self.colors[4 * idx] = 0;
            self.colors[4 * idx + 1] = 0;
            self.colors[4 * idx + 2] = 0;

            let row = idx / self.width;
            let col = idx % self.width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            let z = Complex::new(x, y);
            self.zs[idx] = z;
            self.period_checks[idx] = PeriodCheck::new(z);
            self.cs[idx] = z;
        }
    }

    pub fn step(&mut self, step_size: u64) -> StepResult {
        self.dwell += step_size;

        let mut all_known = true;
        let mut new_colors = false;

        for idx in 0..(self.width * self.height) {
            if let Point::Unknown(dwell) = &mut self.points[idx] {
                all_known = false;

                let c = &self.cs[idx];
                let z = &mut self.zs[idx];
                let period_check = &mut self.period_checks[idx];

                for _ in 0..step_size {
                    let re_squared = z.re * z.re;
                    let im_squared = z.im * z.im;

                    if re_squared + im_squared > 4.0 {
                        self.points[idx] = Point::NotRendered(*dwell);
                        new_colors = true;
                        break;
                    }

                    *dwell += 1;

                    let re_im = z.re * z.im;
                    z.re = re_squared - im_squared + c.re;
                    z.im = 2.0 * re_im + c.im;

                    if *z == period_check.check_against {
                        self.points[idx] = Point::InSet;
                        break;
                    }

                    if *dwell > period_check.dwell_bounds {
                        period_check.check_against = *z;
                        period_check.dwell_bounds *= 2;
                    }
                }
            }
        }

        StepResult {
            all_known,
            new_colors,
        }
    }

    pub fn colorize(&mut self, color_dist: f64) {
        for idx in 0..self.width * self.height {
            if let Point::NotRendered(dwell) = self.points[idx] {
                self.points[idx] = Point::Rendered; // we just rendered it!

                let dwell = dwell as f64 / color_dist;

                let r = dwell;
                let g = dwell + std::f64::consts::FRAC_PI_4;
                let b = dwell + std::f64::consts::FRAC_PI_2;

                let r = r.sin();
                let r = (r * r * 255.0) as u8;

                let g = g.sin();
                let g = (g * g * 255.0) as u8;

                let b = b.sin();
                let b = (b * b * 255.0) as u8;

                self.colors[4 * idx] = r;
                self.colors[4 * idx + 1] = g;
                self.colors[4 * idx + 2] = b;
            }
        }
    }

    pub fn dwell(&self) -> u64 {
        self.dwell
    }

    pub fn colors(&self) -> *const u8 {
        self.colors.as_ptr()
    }
}

#[wasm_bindgen]
pub struct StepResult {
    pub all_known: bool,
    pub new_colors: bool,
}

#[derive(Clone, PartialEq)]
enum Point {
    Unknown(u64),
    InSet,
    NotRendered(u64),
    Rendered,
}

#[derive(Clone)]
struct PeriodCheck {
    check_against: Complex<f64>,
    dwell_bounds: u64,
}

impl PeriodCheck {
    fn new(check_against: Complex<f64>) -> PeriodCheck {
        PeriodCheck {
            check_against,
            dwell_bounds: 8,
        }
    }
}
