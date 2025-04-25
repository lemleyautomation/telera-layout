use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use crate::bindings::*;

/// Configuration settings for rendering text elements.
#[derive(Debug, Clone, Copy)]
pub struct TextConfig {
    /// The color of the text.
    pub color: Color,
    /// Clay does not manage fonts. It is up to the user to assign a unique ID to each font
    /// and provide it via the [`font_id`](Text::font_id) field.
    pub font_id: u16,
    /// The font size of the text.
    pub font_size: u16,
    /// The spacing between letters.
    pub letter_spacing: u16,
    /// The height of each line of text.
    pub line_height: u16,
    /// Defines the text wrapping behavior.
    pub wrap_mode: Clay_TextElementConfigWrapMode,
    /// The alignment of the text.
    pub alignment: Clay_TextAlignment,
}

impl TextConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text color.
    #[inline]
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets the font ID. The user is responsible for assigning unique font IDs.
    #[inline]
    pub fn font_id(&mut self, id: u16) -> &mut Self {
        self.font_id = id;
        self
    }

    /// Sets the font size.
    #[inline]
    pub fn font_size(&mut self, size: u16) -> &mut Self {
        self.font_size = size;
        self
    }

    /// Sets the letter spacing.
    #[inline]
    pub fn letter_spacing(&mut self, spacing: u16) -> &mut Self {
        self.letter_spacing = spacing;
        self
    }

    /// Sets the line height.
    #[inline]
    pub fn line_height(&mut self, height: u16) -> &mut Self {
        self.line_height = height;
        self
    }

    /// Sets the text wrapping mode.
    #[inline]
    pub fn wrap_mode_words(&mut self) -> &mut Self {
        self.wrap_mode = Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_WORDS;
        self
    }
    pub fn wrap_mode_new_lines(&mut self) -> &mut Self {
        self.wrap_mode = Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NEWLINES;
        self
    }
    pub fn wrap_mode_none(&mut self) -> &mut Self {
        self.wrap_mode = Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_NONE;
        self
    }

    /// Sets the text alignment.
    #[inline]
    pub fn alignment_left(&mut self) -> &mut Self {
        self.alignment = Clay_TextAlignment::CLAY_TEXT_ALIGN_LEFT;
        self
    }
    pub fn alignment_right(&mut self) -> &mut Self {
        self.alignment = Clay_TextAlignment::CLAY_TEXT_ALIGN_RIGHT;
        self
    }
    pub fn alignment_center(&mut self) -> &mut Self {
        self.alignment = Clay_TextAlignment::CLAY_TEXT_ALIGN_CENTER;
        self
    }

    pub fn parse(&mut self){}

    /// Finalizes the text configuration
    #[inline]
    pub fn end(self) -> Self {
        self
    }
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            color: Color::default(),
            font_id: 0,
            font_size: 12,
            letter_spacing: 0,
            line_height: 14,
            wrap_mode: Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_WORDS,
            alignment: Clay_TextAlignment::CLAY_TEXT_ALIGN_LEFT
        }
    }
}

impl From<&TextConfig> for Clay_TextElementConfig {
    fn from(value: &TextConfig) -> Self {
        Self {
            textColor: value.color.into(),
            fontId: value.font_id,
            fontSize: value.font_size,
            letterSpacing: value.letter_spacing,
            lineHeight: value.line_height,
            wrapMode: value.wrap_mode as _,
            textAlignment: value.alignment as _,
            userData: std::ptr::null::<usize>() as *mut usize as *mut c_void
        }
    }
}

impl From<Clay_TextElementConfig> for TextConfig {
    fn from(value: Clay_TextElementConfig) -> Self {
        Self {
            color: value.textColor.into(),
            font_id: value.fontId,
            font_size: value.fontSize,
            letter_spacing: value.letterSpacing,
            line_height: value.lineHeight,
            wrap_mode: value.wrapMode,
            alignment: value.textAlignment
        }
    }
}

pub trait MeasureText{
    fn measure_text(&mut self, text: &str, text_config: TextConfig) -> Vec2;
}

pub unsafe extern "C" fn measure_text_c_callback<'a, T>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: *mut core::ffi::c_void,
) -> Clay_Dimensions
where
    T: 'a + MeasureText,
{
    let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        text_slice.chars as *const u8,
        text_slice.length as _,
    ));
    
    let text_config = TextConfig::from(*config);

    let renderer = Rc::from_raw(user_data as *mut RefCell<T>);
    Rc::increment_strong_count(user_data);
    let mut renderer_ref = renderer.borrow_mut();
    renderer_ref.measure_text(text, text_config).into()
}
