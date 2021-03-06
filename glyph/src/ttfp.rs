//! ttf-parser crate specific code. ttf-parser types should not be leaked publicly.
mod outliner;

use crate::{point, Font, GlyphId, InvalidFont, Outline, Rect};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt;
use owned_ttf_parser::AsFaceRef;

impl From<GlyphId> for owned_ttf_parser::GlyphId {
    #[inline]
    fn from(id: GlyphId) -> Self {
        Self(id.0)
    }
}

/// Font data handle stored as a `&[u8]` + parsed data.
/// See [`Font`](trait.Font.html) for more methods.
///
/// Also see the owned version [`FontVec`](struct.FontVec.html).
///
/// # Example
/// ```
/// use ab_glyph::{Font, FontRef};
///
/// # fn main() -> Result<(), ab_glyph::InvalidFont> {
/// let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
///
/// assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct FontRef<'font>(owned_ttf_parser::Face<'font>);

impl fmt::Debug for FontRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontRef")
    }
}

impl<'font> FontRef<'font> {
    /// Creates an `FontRef` from a byte-slice.
    ///
    /// For font collections see
    /// [`FontRef::try_from_slice_and_index`](#method.try_from_slice_and_index).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_slice(data: &'font [u8]) -> Result<Self, InvalidFont> {
        Self::try_from_slice_and_index(data, 0)
    }

    /// Creates an `FontRef` from byte-slice.
    ///
    /// You can set index for font collections. For simple fonts use `0` or
    /// [`FontRef::try_from_slice`](#method.try_from_slice).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// let font =
    ///     FontRef::try_from_slice_and_index(include_bytes!("../../dev/fonts/Exo2-Light.otf"), 0)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_slice_and_index(data: &'font [u8], index: u32) -> Result<Self, InvalidFont> {
        Ok(Self(
            owned_ttf_parser::Face::from_slice(data, index).map_err(|_| InvalidFont)?,
        ))
    }
}

/// Font data handle stored in a `Vec<u8>`  + parsed data.
/// See [`Font`](trait.Font.html) for more methods.
///
/// Also see [`FontRef`](struct.FontRef.html).
///
/// # Example
/// ```
/// use ab_glyph::{Font, FontVec};
///
/// # fn main() -> Result<(), ab_glyph::InvalidFont> {
/// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
/// let font = FontVec::try_from_vec_and_index(owned_font_data, 0)?;
///
/// assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
/// # Ok(()) }
/// ```
pub struct FontVec(owned_ttf_parser::OwnedFace);

impl fmt::Debug for FontVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontVec")
    }
}

impl FontVec {
    /// Creates an `FontVec` from owned data.
    ///
    /// For font collections see
    /// [`FontVec::try_from_vec_and_index`](#method.try_from_vec_and_index).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font = FontVec::try_from_vec(owned_font_data)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_vec(data: Vec<u8>) -> Result<Self, InvalidFont> {
        Self::try_from_vec_and_index(data, 0)
    }

    /// Creates an `FontVec` from owned data.
    ///
    /// You can set index for font collections. For simple fonts use `0` or
    /// [`FontVec::try_from_vec`](#method.try_from_vec).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font = FontVec::try_from_vec_and_index(owned_font_data, 0)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_vec_and_index(data: Vec<u8>, index: u32) -> Result<Self, InvalidFont> {
        Ok(Self(
            owned_ttf_parser::OwnedFace::from_vec(data, index).map_err(|_| InvalidFont)?,
        ))
    }
}

/// Implement `Font` for `Self(AsFontRef)` types.
macro_rules! impl_font {
    ($font:ty) => {
        impl Font for $font {
            #[inline]
            fn units_per_em(&self) -> Option<f32> {
                self.0.as_face_ref().units_per_em().map(f32::from)
            }

            #[inline]
            fn ascent_unscaled(&self) -> f32 {
                f32::from(self.0.as_face_ref().ascender())
            }

            #[inline]
            fn descent_unscaled(&self) -> f32 {
                f32::from(self.0.as_face_ref().descender())
            }

            #[inline]
            fn line_gap_unscaled(&self) -> f32 {
                f32::from(self.0.as_face_ref().line_gap())
            }

            #[inline]
            fn glyph_id(&self, c: char) -> GlyphId {
                let index = self
                    .0
                    .as_face_ref()
                    .glyph_index(c)
                    .map(|id| id.0)
                    .unwrap_or(0);

                let id = GlyphId(index);

                /*if let Some(layers) = self.color_layers(id) {
                    return layers[0].id;
                }*/
                
                id
            }

            #[inline]
            fn h_advance_unscaled(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_face_ref()
                    .glyph_hor_advance(id.into())
                    .expect("Invalid glyph_hor_advance");
                f32::from(advance)
            }

            #[inline]
            fn h_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_face_ref()
                    .glyph_hor_side_bearing(id.into())
                    .expect("Invalid glyph_hor_side_bearing");
                f32::from(advance)
            }

            #[inline]
            fn v_advance_unscaled(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_face_ref()
                    .glyph_ver_advance(id.into())
                    .expect("Invalid glyph_ver_advance");
                f32::from(advance)
            }

            #[inline]
            fn v_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_face_ref()
                    .glyph_ver_side_bearing(id.into())
                    .expect("Invalid glyph_ver_side_bearing");
                f32::from(advance)
            }

            #[inline]
            fn kern_unscaled(&self, first: GlyphId, second: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .kerning_subtables()
                    .filter(|st| st.is_horizontal() && !st.is_variable())
                    .find_map(|st| st.glyphs_kerning(first.into(), second.into()))
                    .map(f32::from)
                    .unwrap_or_default()
            }

            #[inline]
            fn relative_scale(&self, _glyph: GlyphId) -> f32 {
                1.0
            }

            #[inline]
            fn has_color(&self, id: GlyphId) -> bool {
                let face = self.0.as_face_ref();
                face.colr_layers(id.into()).is_some()
            }

            fn color_outlines(&self, id: GlyphId) -> Option<Vec<(Outline,u32)>> {
                let face = self.0.as_face_ref();
                face
                    .colr_layers(id.into())
                    .map(|iter| iter.map(|layer| {
                        let color = face.cpal_color(0, layer.palette_index).unwrap();
                        let color_int = u32::from_be_bytes([color.r,color.g,color.b,color.a]);
                        let outline = self.outline(GlyphId(layer.glyph_id)).unwrap();

                        (outline,color_int)
                    }).collect())
            }

            fn outline(&self, id: GlyphId) -> Option<Outline> {
                let mut outliner = outliner::OutlineCurveBuilder::default();

                let owned_ttf_parser::Rect {
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                } = self
                    .0
                    .as_face_ref()
                    .outline_glyph(id.into(), &mut outliner)?;

                let bounds = Rect {
                    min: point(x_min as f32, y_max as f32),
                    max: point(x_max as f32, y_min as f32),
                };

                Some(Outline {
                    bounds,
                    curves: outliner.take_outline(),
                })
            }

            #[inline]
            fn glyph_count(&self) -> usize {
                self.0.as_face_ref().number_of_glyphs() as _
            }
        }
    };
}

impl_font!(FontRef<'_>);
impl_font!(FontVec);
