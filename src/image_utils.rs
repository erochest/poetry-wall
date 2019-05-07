use image::{ImageBuffer, Rgba};
use crate::font::GlyphVec;
use crate::color::Color;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn create_image(width: u32, height: u32, red: u8, green: u8, blue: u8) -> Image {
    let background = [red as u8, green as u8, blue as u8, 255];
    ImageBuffer::from_pixel(width, height, Rgba(background))
}

pub fn render_glyphs(image: &mut Image, glyphs: &GlyphVec, color: &Color, background: &Color) {
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let pixel_color = color.alpha_composite(background, v);
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba {
                        data: pixel_color.as_array(),
                    },
                )
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use image::{DynamicImage, ImageBuffer, Rgba};

    static WIDTH: u32 = 2880;
    static HEIGHT: u32 = 2560;

    // bench:  31,124,014 ns/iter (+/- 20,982,095)
    // Using this one because the others really only work with a white/gray/black
    // background color. One that fills in all places in the vector with the same
    // number.
    #[bench]
    fn set_background_enumerate_pixels(b: &mut Bencher) {
        let background = [255, 255, 255, 255];
        b.iter(|| {
            let mut image = DynamicImage::new_rgba8(WIDTH, HEIGHT).to_rgba();
            for (_, _, pixel) in image.enumerate_pixels_mut() {
                pixel.data = background;
            }
        });
    }

    // bench:  15,440,571 ns/iter (+/- 2,403,413)
    #[bench]
    fn set_background_from_pixel(b: &mut Bencher) {
        let background = [255 as u8, 255, 255, 255];
        b.iter(|| {
            ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgba(background));
        });
    }

    // 40,771,562 ns/iter (+/- 13,924,953)
    #[bench]
    fn set_background_vec_with_capacity(b: &mut Bencher) {
        let capacity = (4 * WIDTH * HEIGHT) as usize;
        b.iter(|| {
            let mut buffer = Vec::with_capacity(capacity);
            let mut i = 0;
            while i < capacity {
                buffer.push(0);
                i += 1;
            }
            let image: Option<ImageBuffer<Rgba<_>, Vec<_>>> =
                ImageBuffer::from_vec(WIDTH, HEIGHT, buffer);
            image.map(|i| i.is_empty());
        });
    }

    // 22,509,105 ns/iter (+/- 2,739,020)
    // Is it worth special-casing this so I use this when it's white, gray, or black?
    #[bench]
    fn set_background_vec_macro(b: &mut Bencher) {
        let capacity = (4 * WIDTH * HEIGHT) as usize;
        b.iter(|| {
            let buffer = vec![0; capacity];
            let image: Option<ImageBuffer<Rgba<_>, Vec<_>>> =
                ImageBuffer::from_vec(WIDTH, HEIGHT, buffer);
            image.map(|i| i.is_empty());
        });
    }
}

