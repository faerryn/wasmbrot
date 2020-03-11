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
    pub fn bounds(width: usize, height: usize, left: f32, top: f32, pixel_size: f32) -> Wasmbrot {
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
            right: left + width as f32 * pixel_size,
            top,
            down: top - height as f32 * pixel_size,
            pixel_size,
            depth: 0,
            depths: vec![0; width * height],
            in_set: vec![true; width * height],
            zs: cs.clone(),
            cs,
            colors: vec![0; 4 * width * height],
        }
    }

    pub fn step(&mut self, step_size: u32) {
        self.depth += step_size;

        'pixels: for idx in 0..(self.width * self.height) {
            let in_set = &mut self.in_set[idx];

            if *in_set {
                let z = &mut self.zs[idx];

                for partial_step in 1..=step_size {
                    z.mut_square_add(&self.cs[idx]);

                    if z.modulus_squared() > 4.0 {
                        self.depths[idx] += partial_step;
                        *in_set = false;
                        continue 'pixels;
                    }
                }

                self.depths[idx] += step_size;
            }
        }
    }

    pub fn colorize(&mut self) {
        for idx in 0..self.width * self.height {
            let (r, g, b) = if self.in_set[idx] {
                (0, 0, 0)
            } else {
                let point_depth = self.depths[idx] as f32;
                let full_depth = self.depth as f32;

                let hue = point_depth;
                let sat = 1.0;
                let lum = 0.25 + 0.75 * (point_depth / full_depth);

                let (r, g, b) = hsl_to_rgb(hue, sat, lum);

                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
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

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn down(&self) -> f32 {
        self.down
    }

    pub fn pixel_size(&self) -> f32 {
        self.pixel_size
    }
}

fn hsl_to_rgb(hue: f32, saturation: f32, luminance: f32) -> (f32, f32, f32) {
    let hue = hue % 360.0;
    let chroma = (1.0 - (2.0 * luminance - 1.0).abs()) * saturation;
    let cube_face = hue / 60.0;
    let secondary_chroma = chroma * (1.0 - (cube_face % 2.0 - 1.0).abs());
    let m = luminance - chroma / 2.0;

    match cube_face as u32 {
        0 => (chroma + m, secondary_chroma + m, m),
        1 => (secondary_chroma + m, chroma + m, m),
        2 => (m, chroma + m, secondary_chroma + m),
        3 => (m, secondary_chroma + m, chroma + m),
        4 => (secondary_chroma + m, m, chroma + m),
        5 => (chroma + m, m, secondary_chroma + m),
        _ => (0.0, 0.0, 0.0),
    }
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

    #[inline(always)]
    fn modulus_squared(&self) -> f32 {
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
