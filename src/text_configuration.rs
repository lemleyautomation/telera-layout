use std::os::raw::c_void;

use crate::bindings::*;

/// Styling for a run of text, passed to [`LayoutEngine::add_text_element`] and mapped
/// onto clay's `Clay_TextElementConfig`.
///
/// Build one with the chained setters:
///
/// ```
/// use telera_layout::{Color, TextConfig};
///
/// let heading = TextConfig::new()
///     .font_id(1)
///     .font_size(32)
///     .line_height(38)
///     .font_color(Color::rgb(20, 20, 20))
///     .align_center()
///     .end();
/// assert_eq!(heading.font_size, 32);
/// ```
///
/// [`LayoutEngine::add_text_element`]: crate::LayoutEngine::add_text_element
#[derive(Debug, Clone, Copy)]
pub struct TextConfig {
    /// The color of the text.
    pub font_color: Color,
    /// Identifies which font the text-measurement callback and renderer should use.
    /// clay does not manage fonts; the caller assigns the ids. The debug view uses `0`.
    pub font_id: u16,
    /// The font size of the text.
    pub font_size: u16,
    /// The spacing between letters.
    pub letter_spacing: u16,
    /// The height of each line of text.
    pub line_height: u16,
    /// Defines the text wrapping behavior.
    pub wrap_mode: Clay_TextElementConfigWrapMode,
    /// The alignment of wrapped lines within the text bounding box.
    pub alignment: Clay_TextAlignment,
    /// Opaque pointer transparently passed through to `Clay_MeasureText` and to the
    /// resulting TEXT render command (clay's `.userData`). Null unless set via
    /// [`user_data`](Self::user_data).
    pub user_data: *mut c_void,
}

impl TextConfig {
    /// A new config with clay's defaults: black text, font id `0`, font size `12`,
    /// line height `14`, no letter spacing, word wrapping, left alignment, no user data.
    pub const fn new() -> Self {
        Self {
            font_color: Color::rgb(0, 0, 0),
            font_id: 0,
            font_size: 12,
            letter_spacing: 0,
            line_height: 14,
            wrap_mode: Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_WORDS,
            alignment: Clay_TextAlignment::CLAY_TEXT_ALIGN_LEFT,
            user_data: std::ptr::null_mut(),
        }
    }

    /// Sets the text color (TML `font-color`).
    #[inline]
    pub const fn font_color(&mut self, color: Color) -> &mut Self {
        self.font_color = color;
        self
    }

    /// Sets the font id (TML `font-id`). The caller is responsible for assigning
    /// unique ids.
    #[inline]
    pub const fn font_id(&mut self, id: u16) -> &mut Self {
        self.font_id = id;
        self
    }

    /// Sets the font size (TML `font-size`).
    #[inline]
    pub const fn font_size(&mut self, size: u16) -> &mut Self {
        self.font_size = size;
        self
    }

    /// Sets the extra spacing inserted between characters (TML `letter-spacing`).
    #[inline]
    pub const fn letter_spacing(&mut self, spacing: u16) -> &mut Self {
        self.letter_spacing = spacing;
        self
    }

    /// Sets the line height (TML `line-height`).
    #[inline]
    pub const fn line_height(&mut self, height: u16) -> &mut Self {
        self.line_height = height;
        self
    }

    /// Wraps text onto a new line at whitespace when it overflows the available width.
    #[inline]
    pub const fn wrap_mode_words(&mut self) -> &mut Self {
        self.wrap_mode = Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_WORDS;
        self
    }
    /// Breaks text onto a new line only at explicit newline characters, never on
    /// whitespace.
    pub const fn wrap_mode_new_lines(&mut self) -> &mut Self {
        self.wrap_mode = Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NEWLINES;
        self
    }
    /// Disables wrapping entirely; the text is laid out on a single line.
    pub const fn wrap_mode_none(&mut self) -> &mut Self {
        self.wrap_mode = Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NONE;
        self
    }

    /// Aligns wrapped lines to the left edge of the text bounding box (TML `align left`).
    #[inline]
    pub const fn align_left(&mut self) -> &mut Self {
        self.alignment = Clay_TextAlignment::CLAY_TEXT_ALIGN_LEFT;
        self
    }
    /// Aligns wrapped lines to the right edge of the text bounding box (TML `align right`).
    pub const fn align_right(&mut self) -> &mut Self {
        self.alignment = Clay_TextAlignment::CLAY_TEXT_ALIGN_RIGHT;
        self
    }
    /// Centers wrapped lines within the text bounding box (TML `align center`).
    pub const fn align_center(&mut self) -> &mut Self {
        self.alignment = Clay_TextAlignment::CLAY_TEXT_ALIGN_CENTER;
        self
    }

    /// Stores the address of `data` so clay passes it back to the text-measurement
    /// callback and attaches it to the resulting `TEXT` render command (clay's
    /// `.userData`). `data` must outlive every layout pass the config is used in.
    #[inline]
    pub const fn user_data<CustomTextData>(&mut self, data: &CustomTextData) -> &mut Self {
        self.user_data = data as *const CustomTextData as *mut c_void;
        self
    }

    /// Ends the chained-`&mut` builder, returning the finished config by value.
    #[inline]
    pub const fn end(self) -> Self {
        self
    }
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            font_color: Color::default(),
            font_id: 0,
            font_size: 12,
            letter_spacing: 0,
            line_height: 14,
            wrap_mode: Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_WORDS,
            alignment: Clay_TextAlignment::CLAY_TEXT_ALIGN_LEFT,
            user_data: std::ptr::null_mut(),
        }
    }
}

impl From<&TextConfig> for Clay_TextElementConfig {
    fn from(value: &TextConfig) -> Self {
        Self {
            textColor: value.font_color.into(),
            fontId: value.font_id,
            fontSize: value.font_size,
            letterSpacing: value.letter_spacing,
            lineHeight: value.line_height,
            wrapMode: value.wrap_mode as _,
            textAlignment: value.alignment as _,
            userData: value.user_data,
        }
    }
}

impl From<Clay_TextElementConfig> for TextConfig {
    fn from(value: Clay_TextElementConfig) -> Self {
        Self {
            font_color: value.textColor.into(),
            font_id: value.fontId,
            font_size: value.fontSize,
            letter_spacing: value.letterSpacing,
            line_height: value.lineHeight,
            wrap_mode: value.wrapMode,
            alignment: value.textAlignment,
            user_data: value.userData,
        }
    }
}

/// Implemented by the caller's text renderer and handed to
/// [`LayoutEngine::begin_layout`](crate::LayoutEngine::begin_layout). clay calls
/// [`measure_text`](Self::measure_text) whenever it needs the pixel size of a string
/// while computing a layout.
///
/// ```
/// use telera_layout::{MeasureText, TextConfig, Vec2};
///
/// struct MonoFont;
///
/// impl MeasureText for MonoFont {
///     fn measure_text(&mut self, text: &str, _base: &str, config: TextConfig) -> Vec2 {
///         let w = config.font_size as f32 * 0.6;
///         Vec2 { x: text.chars().count() as f32 * w, y: config.line_height as f32 }
///     }
/// }
/// ```
pub trait MeasureText {
    /// Returns the width (`x`) and height (`y`) in pixels that `text` will occupy when
    /// drawn with `text_config`. Called once per unwrapped word; results are cached by
    /// clay until [`LayoutEngine::reset_measure_text_cache`](crate::LayoutEngine::reset_measure_text_cache).
    ///
    /// `base` is the whole string the text element was added with; `text` is a
    /// sub-slice of it (`text.as_ptr()` lies inside `base`), so an implementation
    /// can shape `base` once and answer every word query against that. `base ==
    /// text` when the source length is unknown (e.g. clay's internal space
    /// probe).
    fn measure_text(&mut self, text: &str, base: &str, text_config: TextConfig) -> Vec2;
}

/// FFI trampoline registered with clay's `Clay_SetMeasureTextFunction`. Reconstructs
/// the `&str` and [`TextConfig`] from clay's arguments, casts `user_data` back to the
/// stored `T`, and forwards to [`MeasureText::measure_text`]. Not called directly by
/// user code.
pub unsafe extern "C" fn measure_text_c_callback<'a, T>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: *mut core::ffi::c_void,
) -> Clay_Dimensions
where
    T: 'a + MeasureText,
{
    unsafe {
        let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
            text_slice.chars as *const u8,
            text_slice.length as _,
        ));

        // The full string the slice was cut from, so the renderer can shape it
        // once and serve every word from that. Falls back to `text` when clay
        // didn't record a base length.
        let base = if text_slice.baseLength > 0 && !text_slice.baseChars.is_null() {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                text_slice.baseChars as *const u8,
                text_slice.baseLength as _,
            ))
        } else {
            text
        };

        let text_config = TextConfig::from(*config);

        let renderer: &mut T = &mut *(user_data as *mut T);

        renderer.measure_text(text, base, text_config).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every field of `Clay_TextElementConfig` must be reachable through a builder,
    /// each setter touches exactly one field, and the round trip preserves everything.
    #[test]
    fn builders_cover_every_field_without_clobbering() {
        let marker = 0xABCDu32;
        let mut cfg = TextConfig::new();
        cfg.font_color(Color::rgb(10, 20, 30))
            .font_id(3)
            .font_size(24)
            .letter_spacing(2)
            .line_height(28)
            .wrap_mode_none()
            .align_center()
            .user_data(&marker);

        let c: Clay_TextElementConfig = (&cfg).into();
        assert_eq!(c.textColor.r, 10.0);
        assert_eq!(c.fontId, 3);
        assert_eq!(c.fontSize, 24);
        assert_eq!(c.letterSpacing, 2);
        assert_eq!(c.lineHeight, 28);
        assert_eq!(
            c.wrapMode,
            Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NONE
        );
        assert_eq!(c.textAlignment, Clay_TextAlignment::CLAY_TEXT_ALIGN_CENTER);
        assert_eq!(c.userData as *const u32, &marker as *const u32);

        // Round trip back through TextConfig (as the measure-text callback does).
        let back = TextConfig::from(c);
        assert_eq!(back.font_size, 24);
        assert_eq!(back.line_height, 28);
        assert_eq!(
            back.wrap_mode,
            Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NONE
        );
        assert_eq!(back.alignment, Clay_TextAlignment::CLAY_TEXT_ALIGN_CENTER);
        assert_eq!(back.user_data, c.userData);
    }

    #[test]
    fn setters_are_independent() {
        let mut cfg = TextConfig::new();
        cfg.font_size(50);
        cfg.wrap_mode_new_lines();
        // changing wrap mode must not disturb the earlier font size
        assert_eq!(cfg.font_size, 50);
        assert_eq!(
            cfg.wrap_mode,
            Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NEWLINES
        );
    }
}
