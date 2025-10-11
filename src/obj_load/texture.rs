use image::RgbaImage;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: RgbaImage,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = image::open(path)
            .expect("Failed to load texture")
            .to_rgba8();
        let (width, height) = img.dimensions();
        Self {
            width,
            height,
            data: img,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> (u8, u8, u8) {
        let u = (u.fract() * self.width as f32) as u32;
        let v = ((1.0 - v.fract()) * self.height as f32) as u32;
        let pixel = self
            .data
            .get_pixel(u.min(self.width - 1), v.min(self.height - 1));
        (pixel[0], pixel[1], pixel[2])
    }
}
