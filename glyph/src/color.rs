use crate::GlyphId;

#[derive(Debug)]
pub struct ColorLayer {
    pub id: GlyphId,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
