use rusttype::{Font, Scale, VMetrics};

pub struct Metrics {
    pub font: Font<'static>,
    pub scale: Scale,
    pub font_size: f32,
    pub v_metrics: VMetrics,
    pub top_offset: f32,
    pub left_offset: f32,
}

impl Metrics {
    pub fn new(font: Font<'static>, font_size: f32, top_offset: f32, left_offset: f32) -> Self {
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

    pub fn rescale_to(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.scale = Scale::uniform(font_size);
        self.v_metrics = self.font.v_metrics(self.scale);
    }

    pub fn rescale_by(&mut self, factor: f32) {
        self.rescale_to(self.font_size * factor);
    }
}
