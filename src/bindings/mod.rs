#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::module_inception)]
#![allow(dead_code)]

mod clay;

use std::str::FromStr;

pub use clay::*;

/// An RGBA color with each channel stored as an `f32` in the `0.0..=255.0` range,
/// matching clay's `Clay_Color` convention. Interpretation of the values is left to
/// the renderer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl FromStr for Color {
    type Err = csscolorparser::ParseColorError;

    /// Parses any CSS color string (`"#ff8800"`, `"rgb(255 136 0)"`, `"rebeccapurple"`,
    /// `"hsl(30 100% 50%)"`, …) into a [`Color`] with `0..=255` channels.
    ///
    /// # Examples
    ///
    /// ```
    /// use telera_layout::Color;
    ///
    /// let orange: Color = "#ff8800".parse().unwrap();
    /// assert_eq!((orange.r, orange.g, orange.b, orange.a), (255.0, 136.0, 0.0, 255.0));
    ///
    /// assert!("not a color".parse::<Color>().is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match csscolorparser::parse(s) {
            Ok(color) => Ok(color.to_rgba8().into()),
            Err(e) => Err(e),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 255.0,
        }
    }
}

impl Color {
    /// Builds an opaque color from 8-bit channel values (alpha is set to `255`).
    ///
    /// # Examples
    ///
    /// ```
    /// use telera_layout::Color;
    ///
    /// let c = Color::rgb(18, 52, 86);
    /// assert_eq!((c.r, c.g, c.b, c.a), (18.0, 52.0, 86.0, 255.0));
    /// ```
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: 255.0,
        }
    }

    /// Copies the channel values into clay's `Clay_Color` struct.
    pub const fn to_clay(self) -> Clay_Color {
        Clay_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    /// Packs the four `0..=255` channels into one byte each of a `u32` -
    /// `r` in the low byte through `a` in the high byte, matching WGSL's
    /// `unpack4x8unorm` byte order. Each channel is rounded and clamped to
    /// `0..=255` first.
    ///
    /// Meant for carrying a color through a single flat-interpolated `u32`
    /// vertex attribute (a `` `shader-color-*` `` slot) instead of four
    /// separate `f32` ones - see `telera-app`'s `ResolvedShader::colors`.
    /// Deliberately **not** bit-cast to `f32`: an arbitrary packed RGBA8
    /// value very often lands on a NaN/Inf/denormal `f32` bit pattern, and
    /// those are not guaranteed to survive ordinary (non-flat) vertex
    /// interpolation bit-for-bit on every GPU - carrying the bits as a
    /// genuine `u32` through a flat attribute sidesteps that entirely.
    ///
    /// # Examples
    ///
    /// ```
    /// use telera_layout::Color;
    ///
    /// let c = Color::rgb(0x12, 0x34, 0x56);
    /// assert_eq!(c.pack_rgba8(), 0xFF_56_34_12);
    /// ```
    pub fn pack_rgba8(&self) -> u32 {
        let byte = |v: f32| v.round().clamp(0.0, 255.0) as u32;
        byte(self.r) | (byte(self.g) << 8) | (byte(self.b) << 16) | (byte(self.a) << 24)
    }
}

impl Into<Clay_Color> for Color {
    fn into(self) -> Clay_Color {
        Clay_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl Into<Color> for Clay_Color {
    fn into(self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self {
            r: value[0] as f32,
            g: value[1] as f32,
            b: value[2] as f32,
            a: value[3] as f32,
        }
    }
}

/// An axis-aligned rectangle in layout space: top-left corner at `(x, y)` with the
/// given `width` and `height`, all in pixels relative to the root of the layout.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Into<BoundingBox> for Clay_BoundingBox {
    fn into(self) -> BoundingBox {
        BoundingBox {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

/// A two-component `f32` vector. Used for positions, offsets and, when returned from
/// [`MeasureText::measure_text`](crate::MeasureText::measure_text), the width (`x`) and
/// height (`y`) of a measured string.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Into<Vec2> for Clay_Dimensions {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.width,
            y: self.height,
        }
    }
}

impl Into<Clay_Dimensions> for Vec2 {
    fn into(self) -> Clay_Dimensions {
        Clay_Dimensions {
            width: self.x,
            height: self.y,
        }
    }
}

pub type ElementID = Clay_ElementId;

/// Defines individual corner radii for an element.
#[derive(Debug, Clone)]
pub struct CornerRadii {
    /// The radius for the top-left corner.
    pub top_left: f32,
    /// The radius for the top-right corner.
    pub top_right: f32,
    /// The radius for the bottom-left corner.
    pub bottom_left: f32,
    /// The radius for the bottom-right corner.
    pub bottom_right: f32,
}

impl From<Clay_CornerRadius> for CornerRadii {
    fn from(value: Clay_CornerRadius) -> Self {
        Self {
            top_left: value.topLeft,
            top_right: value.topRight,
            bottom_left: value.bottomLeft,
            bottom_right: value.bottomRight,
        }
    }
}

/// Defines the border width for each side of an element.
#[derive(Debug, Clone)]
pub struct BorderWidth {
    /// Border width on the left side.
    pub left: u16,
    /// Border width on the right side.
    pub right: u16,
    /// Border width on the top side.
    pub top: u16,
    /// Border width on the bottom side.
    pub bottom: u16,
    /// Border width between child elements.
    pub between_children: u16,
}

impl Into<BorderWidth> for Clay_BorderWidth {
    fn into(self) -> BorderWidth {
        BorderWidth {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
            between_children: self.betweenChildren,
        }
    }
}

/// Represents a rectangle with a specified color and corner radii.
#[derive(Debug, Clone)]
pub struct Rectangle<'render_pass, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// The id of the element that produced this render command, passed through from
    /// its [`ElementConfiguration`](crate::ElementConfiguration).
    pub id: u32,
    /// Stacking order for this command. The command array is already sorted by it;
    /// higher values are drawn on top.
    pub z_index: i16,
    /// The value attached with
    /// [`ElementConfiguration::custom_layout_settings`](crate::ElementConfiguration::custom_layout_settings),
    /// borrowed for the duration of the render pass; `None` if the element set none.
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// The fill color of the rectangle.
    pub color: Color,
    /// The corner radii for rounded edges.
    pub corner_radii: CornerRadii,
}

/// Represents a border with a specified color, width, and corner radii.
#[derive(Debug, Clone)]
pub struct Border<'render_pass, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// The id of the element that produced this render command, passed through from
    /// its [`ElementConfiguration`](crate::ElementConfiguration).
    pub id: u32,
    /// Stacking order for this command. The command array is already sorted by it;
    /// higher values are drawn on top.
    pub z_index: i16,
    /// The value attached with
    /// [`ElementConfiguration::custom_layout_settings`](crate::ElementConfiguration::custom_layout_settings),
    /// borrowed for the duration of the render pass; `None` if the element set none.
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// The color applied to every side with a non-zero width.
    pub color: Color,
    /// The corner radii for rounded border edges.
    pub corner_radii: CornerRadii,
    /// The width of the border on each side.
    pub width: BorderWidth,
}

/// Represents a text element with styling attributes.
#[derive(Debug, Clone)]
pub struct Text<'render_pass, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// The id of the element that produced this render command, passed through from
    /// its [`ElementConfiguration`](crate::ElementConfiguration).
    pub id: u32,
    /// Stacking order for this command. The command array is already sorted by it;
    /// higher values are drawn on top.
    pub z_index: i16,
    /// The value attached with
    /// [`ElementConfiguration::custom_layout_settings`](crate::ElementConfiguration::custom_layout_settings),
    /// borrowed for the duration of the render pass; `None` if the element set none.
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// The text content.
    pub text: &'render_pass str,
    /// The color of the text.
    pub color: Color,
    /// The ID of the font used.
    pub font_id: u16,
    /// The font size.
    pub font_size: u16,
    /// The spacing between letters.
    pub letter_spacing: u16,
    /// The line height.
    pub line_height: u16,
}

/// An `IMAGE` render command: a tint color, corner radii and the caller's image data.
#[derive(Debug, Clone)]
pub struct Image<'render_pass, ImageElementData, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// The id of the element that produced this render command, passed through from
    /// its [`ElementConfiguration`](crate::ElementConfiguration).
    pub id: u32,
    /// Stacking order for this command. The command array is already sorted by it;
    /// higher values are drawn on top.
    pub z_index: i16,
    /// The value attached with
    /// [`ElementConfiguration::custom_layout_settings`](crate::ElementConfiguration::custom_layout_settings),
    /// borrowed for the duration of the render pass; `None` if the element set none.
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// Tint color for the image; `(0, 0, 0, 0)` means "untinted".
    pub background_color: Color,
    /// The image data attached with
    /// [`ElementConfiguration::image`](crate::ElementConfiguration::image),
    /// borrowed for the duration of the render pass.
    pub data: &'render_pass ImageElementData,
}

/// A `CUSTOM` render command: a background color, corner radii and the caller's data,
/// left for the renderer to draw however it likes.
#[derive(Debug, Clone)]
pub struct Custom<'render_pass, CustomElementData, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// The id of the element that produced this render command, passed through from
    /// its [`ElementConfiguration`](crate::ElementConfiguration).
    pub id: u32,
    /// Stacking order for this command. The command array is already sorted by it;
    /// higher values are drawn on top.
    pub z_index: i16,
    /// The value attached with
    /// [`ElementConfiguration::custom_layout_settings`](crate::ElementConfiguration::custom_layout_settings),
    /// borrowed for the duration of the render pass; `None` if the element set none.
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// The background color of the custom element.
    pub background_color: Color,
    /// The corner radii for rounded edges.
    pub corner_radii: CornerRadii,
    /// The data attached with
    /// [`ElementConfiguration::custom_element`](crate::ElementConfiguration::custom_element),
    /// borrowed for the duration of the render pass.
    pub data: &'render_pass CustomElementData,
}

// Each of these reads fields out of `Clay_RenderCommand.renderData`, which is a C union.
// The access is only sound when `value.commandType` matches the variant being built;
// callers (see `LayoutEngine::end_layout`) select the impl by matching on `commandType`
// first.
impl<'render_pass, CustomLayoutSettings> From<&Clay_RenderCommand>
    for Rectangle<'render_pass, CustomLayoutSettings>
{
    fn from(value: &Clay_RenderCommand) -> Self {
        Rectangle {
            bounding_box: value.boundingBox.into(),
            id: value.id,
            z_index: value.zIndex,
            custom_layout_settings: unsafe {
                value.userData.cast::<CustomLayoutSettings>().as_ref()
            },
            color: unsafe { value.renderData.rectangle.backgroundColor.into() },
            corner_radii: unsafe { value.renderData.rectangle.cornerRadius.into() },
        }
    }
}

impl<'render_pass, CustomLayoutSettings> From<&Clay_RenderCommand>
    for Border<'render_pass, CustomLayoutSettings>
{
    fn from(value: &Clay_RenderCommand) -> Self {
        Border {
            bounding_box: value.boundingBox.into(),
            id: value.id,
            z_index: value.zIndex,
            custom_layout_settings: unsafe {
                value.userData.cast::<CustomLayoutSettings>().as_ref()
            },
            color: unsafe { value.renderData.border.color.into() },
            corner_radii: unsafe { value.renderData.border.cornerRadius.into() },
            width: unsafe { value.renderData.border.width.into() },
        }
    }
}

impl<'render_pass, CustomLayoutSettings> From<&Clay_RenderCommand>
    for Text<'render_pass, CustomLayoutSettings>
{
    fn from(value: &Clay_RenderCommand) -> Self {
        Text {
            bounding_box: value.boundingBox.into(),
            id: value.id,
            z_index: value.zIndex,
            custom_layout_settings: unsafe {
                value.userData.cast::<CustomLayoutSettings>().as_ref()
            },
            text: unsafe {
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    value.renderData.text.stringContents.chars as *const u8,
                    value.renderData.text.stringContents.length as _,
                ))
            },
            color: unsafe { value.renderData.text.textColor.into() },
            font_id: unsafe { value.renderData.text.fontId },
            font_size: unsafe { value.renderData.text.fontSize },
            letter_spacing: unsafe { value.renderData.text.letterSpacing },
            line_height: unsafe { value.renderData.text.lineHeight },
        }
    }
}

impl From<&Clay_RenderCommand> for BoundingBox {
    fn from(value: &Clay_RenderCommand) -> Self {
        value.boundingBox.into()
    }
}

impl<'render_pass, ImageElementData, CustomLayoutSettings> From<&Clay_RenderCommand>
    for Image<'render_pass, ImageElementData, CustomLayoutSettings>
{
    fn from(value: &Clay_RenderCommand) -> Self {
        Image {
            bounding_box: value.boundingBox.into(),
            id: value.id,
            z_index: value.zIndex,
            custom_layout_settings: unsafe {
                value.userData.cast::<CustomLayoutSettings>().as_ref()
            },
            background_color: unsafe { value.renderData.image.backgroundColor.into() },
            data: unsafe { &*value.renderData.image.imageData.cast() },
        }
    }
}

impl<CustomElementData, CustomLayoutSettings> From<&Clay_RenderCommand>
    for Custom<'_, CustomElementData, CustomLayoutSettings>
{
    fn from(value: &Clay_RenderCommand) -> Self {
        Custom {
            bounding_box: value.boundingBox.into(),
            id: value.id,
            z_index: value.zIndex,
            custom_layout_settings: unsafe {
                value.userData.cast::<CustomLayoutSettings>().as_ref()
            },
            background_color: unsafe { value.renderData.custom.backgroundColor.into() },
            corner_radii: unsafe { value.renderData.custom.cornerRadius.into() },
            data: unsafe { &*value.renderData.custom.customData.cast() },
        }
    }
}

/// One drawing instruction produced by [`LayoutEngine::end_layout`](crate::LayoutEngine::end_layout).
///
/// The vector returned from `end_layout` is already ordered back-to-front, so drawing
/// the commands in sequence yields correct output. `None` commands should be skipped;
/// `ScissorStart` / `ScissorEnd` bracket a clipped region.
#[derive(Debug, Clone)]
pub enum RenderCommand<'render_pass, ImageElementData, CustomElementData, CustomLayoutSettings> {
    None,
    Rectangle(Rectangle<'render_pass, CustomLayoutSettings>),
    Border(Border<'render_pass, CustomLayoutSettings>),
    Text(Text<'render_pass, CustomLayoutSettings>),
    ScissorStart(BoundingBox),
    ScissorEnd,
    Image(Image<'render_pass, ImageElementData, CustomLayoutSettings>),
    Custom(Custom<'render_pass, CustomElementData, CustomLayoutSettings>),
}
