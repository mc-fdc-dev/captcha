use image::RgbaImage;
use imageproc::drawing::draw_text_mut;
use rand::Rng;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const WIDTH: u32 = 256;
const HEIGHT: u32 = 128;

pub struct Captcha {
    image: RgbaImage,
    color: image::Rgba<u8>,
}

impl Captcha {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..255);
        let g: u8 = rng.gen_range(0..255);
        let b: u8 = rng.gen_range(0..255);
        Self {
            image: RgbaImage::from_fn(WIDTH, HEIGHT, |_x, _y| {
                // all pixels are white
                image::Rgba([255, 255, 255, 255])
            }),
            color: image::Rgba([r, g, b, 255]),
        }
    }

    fn random_color(&mut self) {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..255);
        let g: u8 = rng.gen_range(0..255);
        let b: u8 = rng.gen_range(0..255);
        self.color = image::Rgba([r, g, b, 255]);
    }

    fn draw_text(&mut self, text: &str) -> Result<()> {
        let mut rng = rand::thread_rng();
        for (i, c) in text.chars().enumerate() {
            // random height place
            let height: i32 = rng.gen_range(0..(HEIGHT - 32) as i32);
            let scale = rusttype::Scale::uniform(42.0);
            let font =
                rusttype::Font::try_from_bytes(include_bytes!("../fonts/Roboto-Regular.ttf"))
                    .unwrap();
            // color is gray
            // let color = image::Rgba([128, 128, 128, 255]);
            draw_text_mut(
                &mut self.image,
                self.color,
                (30 * i as i32) + 32,
                height as i32,
                scale,
                &font,
                &c.to_string(),
            );
        }
        Ok(())
    }

    fn draw_line(&mut self) {
        // create three line
        let mut rng = rand::thread_rng();
        // thickness is 2
        for _ in 0..15 {
            let x1: i32 = rng.gen_range(0..WIDTH as i32);
            let y1: i32 = rng.gen_range(0..HEIGHT as i32);
            let x2: i32 = rng.gen_range(0..WIDTH as i32);
            let y2: i32 = rng.gen_range(0..HEIGHT as i32);
            imageproc::drawing::draw_line_segment_mut(
                &mut self.image,
                (x1 as f32, y1 as f32),
                (x2 as f32, y2 as f32),
                self.color,
            );
        }
    }

    pub fn generate(&mut self) -> Result<(String, RgbaImage)> {
        let mut rng = rand::thread_rng();
        let text: String = (0..6).map(|_| rng.gen_range(0..9).to_string()).collect();
        self.draw_text(&text)?;
        self.draw_line();
        Ok((text, self.image.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let mut captcha = Captcha::new();
        let (text, image) = captcha.generate().unwrap();
        // captcha.random_color();
        image.save("test.png").unwrap();
        assert_eq!(result, 4);
    }
}
