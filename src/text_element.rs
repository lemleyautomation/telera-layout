use std::os::raw::c_void;

use crate::bindings::clay::*;

pub struct TextElementConfig {
    inner: *mut Clay_TextElementConfig,
}

impl From<TextElementConfig> for *mut Clay_TextElementConfig {
    fn from(value: TextElementConfig) -> Self {
        value.inner
    }
}

/// Configuration settings for rendering text elements.
#[derive(Debug, Clone, Copy)]
pub struct TextConfig {
    /// The color of the text.
    pub color: [f32;4],
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
    /// Creates a new `TextConfig` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text color.
    #[inline]
    pub fn color(&mut self, r:f32, g:f32, b:f32, a:f32) -> &mut Self {
        self.color = [r,g,b,a];
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
    pub fn wrap_mode(&mut self, mode: Clay_TextElementConfigWrapMode) -> &mut Self {
        self.wrap_mode = mode;
        self
    }

    /// Sets the text alignment.
    #[inline]
    pub fn alignment(&mut self, alignment: Clay_TextAlignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    /// Finalizes the text configuration and stores it in memory.
    #[inline]
    pub fn end(&self) -> TextElementConfig {
        let memory = unsafe { Clay__StoreTextElementConfig((*self).into()) };
        TextElementConfig { inner: memory }
    }
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            color: [0.0,0.0,0.0,0.0],
            font_id: 0,
            font_size: 0,
            letter_spacing: 0,
            line_height: 0,
            wrap_mode: Clay_TextElementConfigWrapMode::CLAY_TEXT_WRAP_WORDS,
            alignment: Clay_TextAlignment::CLAY_TEXT_ALIGN_LEFT
        }
    }
}

impl From<TextConfig> for Clay_TextElementConfig {
    fn from(value: TextConfig) -> Self {
        Self {
            textColor: Clay_Color{
                r: value.color[0],
                g: value.color[1],
                b: value.color[2],
                a: value.color[3],
            },
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

pub unsafe extern "C" fn measure_text_trampoline_user_data<'a, F, T>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: *mut core::ffi::c_void,
) -> Clay_Dimensions
where
    F: Fn(&str, &TextConfig, &'a mut T) -> (f32,f32) + 'a,
    T: 'a,
{
    let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        text_slice.chars as *const u8,
        text_slice.length as _,
    ));

    let closure_and_data: &mut (F, T) = &mut *(user_data as *mut (F, T));
    let text_config = TextConfig::from(*config);
    let (callback, data) = closure_and_data;
    callback(text, &text_config, data).into()
}