#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::module_inception)]
#![allow(dead_code)]

mod clay;

pub use clay::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 255.0 }
    }
}

impl Into<Clay_Color> for Color{
    fn into(self) -> Clay_Color {
        Clay_Color { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

impl Into<Color> for Clay_Color{
    fn into(self) -> Color {
        Color { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

impl Into<Color> for [u8;4] {
    fn into(self) -> Color {
        Color { r: self[0] as f32, g: self[1] as f32, b: self[2] as f32, a: self[3] as f32 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Into<BoundingBox> for Clay_BoundingBox {
    fn into(self) -> BoundingBox {
        BoundingBox { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2{
    pub x: f32,
    pub y: f32
}

impl Into<Vec2> for Clay_Dimensions {
    fn into(self) -> Vec2 {
        Vec2 { x: self.width, y: self.height }
    }
}

impl Into<Clay_Dimensions> for Vec2 {
    fn into(self) -> Clay_Dimensions {
        Clay_Dimensions { width: self.x, height: self.y }
    }
}

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
        BorderWidth { left: self.left, right: self.right, top: self.top, bottom: self.bottom, between_children: self.betweenChildren }
    }
}

/// Represents a rectangle with a specified color and corner radii.
#[derive(Debug, Clone)]
pub struct Rectangle<'render_pass, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// A unique identifier for the render command.
    pub id: u32,
    /// The z-index determines the stacking order of elements.
    /// Higher values are drawn above lower values.
    pub z_index: i16,
    /// Custom Layout data passed through the engine untouched.
    /// This can be used to extend the engine with features
    /// not yet implemented
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
    /// A unique identifier for the render command.
    pub id: u32,
    /// The z-index determines the stacking order of elements.
    /// Higher values are drawn above lower values.
    pub z_index: i16,
    /// Custom Layout data passed through the engine untouched.
    /// This can be used to extend the engine with features
    /// not yet implemented
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// The text content.
    /// The border color.
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
    /// A unique identifier for the render command.
    pub id: u32,
    /// The z-index determines the stacking order of elements.
    /// Higher values are drawn above lower values.
    pub z_index: i16,
    /// Custom Layout data passed through the engine untouched.
    /// This can be used to extend the engine with features
    /// not yet implemented
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

/// Represents an image with defined dimensions and data.
#[derive(Debug, Clone)]
pub struct Image<'render_pass, ImageElementData, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// A unique identifier for the render command.
    pub id: u32,
    /// The z-index determines the stacking order of elements.
    /// Higher values are drawn above lower values.
    pub z_index: i16,
    /// Custom Layout data passed through the engine untouched.
    /// This can be used to extend the engine with features
    /// not yet implemented
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// Background color
    pub background_color: Color,
    /// The dimensions of the image.
    pub dimensions: Vec2,
    /// A pointer to the image data.
    pub data: &'render_pass ImageElementData,
}

/// Represents a custom element with a background color, corner radii, and associated data.
#[derive(Debug, Clone)]
pub struct Custom<'render_pass, CustomElementData, CustomLayoutSettings> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// A unique identifier for the render command.
    pub id: u32,
    /// The z-index determines the stacking order of elements.
    /// Higher values are drawn above lower values.
    pub z_index: i16,
    /// Custom Layout data passed through the engine untouched.
    /// This can be used to extend the engine with features
    /// not yet implemented
    pub custom_layout_settings: Option<&'render_pass CustomLayoutSettings>,
    /// The background color of the custom element.
    pub background_color: Color,
    /// The corner radii for rounded edges.
    pub corner_radii: CornerRadii,
    /// A pointer to additional custom data.
    pub data: &'render_pass CustomElementData,
}

impl<'render_pass, CustomLayoutSettings> From<&Clay_RenderCommand> for Rectangle<'render_pass, CustomLayoutSettings> {
    fn from(value: &Clay_RenderCommand) -> Self {
        Rectangle { 
            bounding_box: value.boundingBox.into(), 
            id: value.id, 
            z_index: value.zIndex,
            custom_layout_settings: unsafe { Some(&*value.userData.cast()) },
            color: unsafe { value.renderData.rectangle.backgroundColor.into() }, 
            corner_radii: unsafe { value.renderData.rectangle.cornerRadius.into() }
        }
    }
}

impl<'render_pass, CustomLayoutSettings> From<&Clay_RenderCommand> for Border<'render_pass, CustomLayoutSettings> {
    fn from(value: &Clay_RenderCommand) -> Self {
        Border { 
            bounding_box: value.boundingBox.into(), 
            id: value.id, 
            z_index: value.zIndex, 
            custom_layout_settings: unsafe { Some(&*value.userData.cast()) },
            color: unsafe { value.renderData.border.color.into() }, 
            corner_radii: unsafe { value.renderData.border.cornerRadius.into() }, 
            width: unsafe { value.renderData.border.width.into() } 
        }
    }
}

impl<'render_pass, CustomLayoutSettings> From<&Clay_RenderCommand> for Text<'render_pass, CustomLayoutSettings> {
    fn from(value: &Clay_RenderCommand) -> Self {
        Text { 
            bounding_box: value.boundingBox.into(), 
            id: value.id, 
            z_index: value.zIndex,
            custom_layout_settings: unsafe { Some(&*value.userData.cast()) },
            text: unsafe {
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    value.renderData.text.stringContents.chars as *const u8,
                    value.renderData.text.stringContents.length as _,
                ))
            }, 
            color: unsafe { value.renderData.text.textColor.into()  }, 
            font_id: unsafe { value.renderData.text.fontId  }, 
            font_size: unsafe { value.renderData.text.fontSize  }, 
            letter_spacing: unsafe { value.renderData.text.letterSpacing  }, 
            line_height: unsafe { value.renderData.text.lineHeight  }, 
        }
    }
}

impl<'render_pass> From<&Clay_RenderCommand> for BoundingBox {
    fn from(value: &Clay_RenderCommand) -> Self {
        value.boundingBox.clone().into()
    }
}

impl<'render_pass, ImageElementData, CustomLayoutSettings> From<&Clay_RenderCommand> for Image<'render_pass, ImageElementData, CustomLayoutSettings>{
    fn from(value: &Clay_RenderCommand) -> Self {
        Image { 
            bounding_box: value.boundingBox.into(), 
            id: value.id, 
            z_index: value.zIndex, 
            custom_layout_settings: unsafe { Some(&*value.userData.cast()) },
            background_color: unsafe { value.renderData.image.backgroundColor.into() }, 
            dimensions: Vec2 { x: 0.0, y: 0.0 }, 
            data: unsafe { &*value.renderData.image.imageData.cast() }
        }
    }
}

impl<CustomElementData, CustomLayoutSettings> From<&Clay_RenderCommand> for Custom<'_, CustomElementData, CustomLayoutSettings> {
    fn from(value: &Clay_RenderCommand) -> Self {
        Custom {
            bounding_box: value.boundingBox.into(), 
            id: value.id, 
            z_index: value.zIndex,
            custom_layout_settings: unsafe { Some(&*value.userData.cast()) },
            background_color: unsafe { value.renderData.custom.backgroundColor.into() }, 
            corner_radii: unsafe { value.renderData.custom.cornerRadius.into() }, 
            data: unsafe { &*value.renderData.custom.customData.cast() },
        }
    }
}

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
