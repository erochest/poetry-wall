use rusttype::{Font, point, Scale, VMetrics};

use crate::bounding_box::BoundingBox;
use crate::font::GlyphVec;
use crate::options::PoetryWallOptions;
use crate::poem::Poem;

pub struct Metrics<'a> {
    pub font: Font<'a>,
    pub scale: Scale,
    pub font_size: f32,
    pub v_metrics: VMetrics,
    pub top_offset: f32,
    pub left_offset: f32,
}

impl<'a> Metrics<'a> {
    pub fn new(font: Font<'a>, font_size: f32, top_offset: f32, left_offset: f32) -> Self {
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);
        Metrics {
            font,
            scale,
            font_size,
            v_metrics,
            top_offset,
            left_offset,
        }
    }

    pub fn compute_metrics(options: &PoetryWallOptions, poem: &Poem, font: Font<'a>) -> Self {
        let mut metrics = Metrics::new(font, options.font_size, 0.0, 0.0);
        let bounding_box = loop {
            let glyphs = metrics.create_glyphs(poem.lines());
            let bb = BoundingBox::compute_bounding_box(&glyphs);
            if ((bb.bottom - bb.top) as u32) < options.dimensions.height {
                break bb;
            }
            metrics.rescale_by(0.9);
        };
        let rendered_height = (bounding_box.bottom - bounding_box.top) as u32;
        let rendered_width = (bounding_box.right - bounding_box.left) as u32;

        metrics.top_offset = options
            .top
            .map(|v| v as f32)
            .unwrap_or_else(|| 0.33 * (options.dimensions.height - rendered_height) as f32);
        metrics.left_offset = options
            .left
            .map(|v| v as f32)
            .unwrap_or_else(|| 0.25 * (options.dimensions.width - rendered_width) as f32);

        metrics
    }

    pub fn rescale_to(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.scale = Scale::uniform(font_size);
        self.v_metrics = self.font.v_metrics(self.scale);
    }

    pub fn rescale_by(&mut self, factor: f32) {
        self.rescale_to(self.font_size * factor);
    }

    pub fn create_glyphs(&self, lines: &Vec<String>) -> GlyphVec {
        let mut glyphs = Vec::new();
        let mut top = self.top_offset + self.v_metrics.ascent;
        let line_height =
            self.v_metrics.ascent + self.v_metrics.descent.abs() + self.v_metrics.line_gap;
        let left = self.left_offset;
        for text in lines {
            let mut line_glyphs = self
                .font
                .layout(&text, self.scale, point(left, top))
                .collect::<Vec<_>>();
            top += line_height;
            glyphs.append(&mut line_glyphs);
        }
        glyphs
    }
}
