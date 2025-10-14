use image::RgbaImage;

/// A texture that can be sampled to get a color at a given UV coordinate.
#[derive(Clone, Debug)]
pub struct Texture {
    /// The width of the texture in pixels.
    pub width: u32,
    /// The height of the texture in pixels.
    pub height: u32,
    /// The raw pixel data of the texture.
    pub data: RgbaImage,
}

impl Texture {
    /// Loads a texture from a file.
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

    /// Samples the texture at a given UV coordinate.
    ///
    /// The U coordinate is the horizontal coordinate, and the V coordinate is the
    /// vertical coordinate. Both coordinates are in the range [0, 1].
    pub fn sample(&self, u: f32, v: f32) -> (u8, u8, u8) {
        let u = (u.fract() * self.width as f32) as u32;
        let v = ((1.0 - v.fract()) * self.height as f32) as u32;
        let pixel = self
            .data
            .get_pixel(u.min(self.width - 1), v.min(self.height - 1));
        (pixel[0], pixel[1], pixel[2])
    }
}