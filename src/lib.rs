use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasmbrot {
    width: usize,
    height: usize,
    left: f32,
    right: f32,
    top: f32,
    down: f32,
    pixel_size: f32,
    depth: u32,
    depths: Vec<u32>,
    in_set: Vec<bool>,
    zs: Vec<Complex>,
    cs: Vec<Complex>,
    colors: Vec<u8>,
}

#[wasm_bindgen]
impl Wasmbrot {
    pub fn new(width: usize, height: usize, center_x: f32, center_y: f32, scale: f32) -> Wasmbrot {
        let pixel_size = scale * 2.0 / width.min(height) as f32;
        let left = center_x - pixel_size * width as f32 / 2.0;
        let right = center_x + width as f32 * pixel_size / 2.0;
        let top = center_y + pixel_size * height as f32 / 2.0;
        let down = center_y - height as f32 * pixel_size / 2.0;

        let cs: Vec<_> = (0..width * height)
            .map(|idx| {
                let row = idx / width;
                let col = idx % width;

                let x = left + col as f32 * pixel_size;
                let y = top - row as f32 * pixel_size;

                Complex::new(x, y)
            })
            .collect();

        Wasmbrot {
            width,
            height,
            left,
            right,
            top,
            down,
            pixel_size,
            depth: 0,
            depths: vec![0; width * height],
            in_set: vec![true; width * height],
            zs: cs.clone(),
            cs,
            colors: vec![0; 4 * width * height],
        }
    }

    pub fn tick(&mut self) {
        self.depth += 1;

        for idx in 0..(self.width * self.height) {
            let in_set = &mut self.in_set[idx];

            if *in_set {
                let z = &mut self.zs[idx];

                if z.abs_square() > 4.0 {
                    *in_set = false;
                } else {
                    z.square();
                    *z += &self.cs[idx];

                    self.depths[idx] += 1;
                }
            }
        }
    }

    pub fn colorize(&mut self) {
        for idx in 0..(self.width * self.height) {
            let point_depth = self.depths[idx];

            let (red, gre, blu) = if self.in_set[idx] {
                (0.0, 0.0, 0.0)
            } else {
                let hue = point_depth as f32;
                let sat = 1.0;
                let lum = (point_depth as f32 / self.depth as f32).sqrt();

                hsl_to_rgb(hue, sat, lum)
            };

            self.colors[4 * idx] = (red * 255.0) as u8;
            self.colors[4 * idx + 1] = (gre * 255.0) as u8;
            self.colors[4 * idx + 2] = (blu * 255.0) as u8;
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

fn hsl_to_rgb(hue: f32, saturation: f32, luminance: f32) -> (f32, f32, f32) {
    let chroma = (1.0 - (2.0 * luminance - 1.0).abs()) * saturation;
    let hue_cube_side = hue / 60.0;
    let second_largest = chroma * (1.0 - (hue_cube_side % 2.0 - 1.0).abs());

    let (red_axis, green_axis, blue_axis) = match hue_cube_side.ceil() as u8 {
        0..=1 => (chroma, second_largest, 0.0),
        2 => (second_largest, chroma, 0.0),
        3 => (0.0, chroma, second_largest),
        4 => (0.0, second_largest, chroma),
        5 => (second_largest, 0.0, chroma),
        6 => (chroma, 0.0, second_largest),
        _ => (0.0, 0.0, 0.0),
    };

    let m = luminance - chroma / 2.0;

    (red_axis + m, green_axis + m, blue_axis + m)
}

#[derive(Clone)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    fn new(real: f32, imag: f32) -> Complex {
        Complex { real, imag }
    }

    fn abs_square(&self) -> f32 {
        self.real * self.real + self.imag * self.imag
    }

    fn square(&mut self) {
        let real = self.real * self.real - self.imag * self.imag;
        let imag = 2.0 * (self.real * self.imag);
        self.real = real;
        self.imag = imag;
    }
}

impl std::ops::AddAssign<&Complex> for Complex {
    fn add_assign(&mut self, rhs: &Complex) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}
