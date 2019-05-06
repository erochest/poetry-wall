use crate::font::GlyphVec;

pub struct BoundingBox {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

impl BoundingBox {
    pub fn compute_bounding_box(glyphs: &GlyphVec) -> Self {
        let mut bb = BoundingBox {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        };

        for glyph in glyphs {
            if let Some(glyph_bb) = glyph.pixel_bounding_box() {
                bb.top = bb.top.min(glyph_bb.min.y);
                bb.left = bb.left.min(glyph_bb.min.x);
                bb.bottom = bb.bottom.max(glyph_bb.max.y);
                bb.right = bb.right.max(glyph_bb.max.x);
            }
        }

        bb
    }
}
