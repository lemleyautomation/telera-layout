use std::os::raw::c_void;

use crate::bindings::*;

#[derive(Default, Clone, Copy)]
pub struct ElementConfiguration{
    decleration: Clay_ElementDeclaration
}

impl ElementConfiguration{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn id(&mut self, label: &str) -> &mut Self {
        self.decleration.id = unsafe {
            Clay__HashString(
                Clay_String { 
                    isStaticallyAllocated: true, 
                    length: label.len() as i32, 
                    chars: label.as_ptr() as *const _
                },
                0,
                0
            ) 
        };
        self
    }
    pub fn grow_all(&mut self) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        };
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        };
        self
    }
    pub fn x_grow(&mut self) -> &mut Self {
        //println!("x-grow function");
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        };
        self
    }
    pub fn x_grow_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    pub fn x_grow_min_max(&mut self, min: f32, max: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    pub fn y_grow(&mut self) -> &mut Self {
        //println!("y-grow-function");
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        };
        self
    }
    pub fn y_grow_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    pub fn y_grow_min_max(&mut self, min: f32, max: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    pub fn x_fit(&mut self) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        };
        self
    }
    pub fn x_fit_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    pub fn x_fit_min_max(&mut self, min: f32,max: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    pub fn y_fit(&mut self) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        };
        self
    }
    pub fn y_fit_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    pub fn y_fit_min_max(&mut self, min: f32,max: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    pub fn x_fixed(&mut self, size:f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: size, max: size },
            },
        };
        self
    }
    pub fn y_fixed(&mut self, size:f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: size, max: size },
            },
        };
        self
    }
    pub fn x_percent(&mut self, percent:f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_PERCENT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: percent, max: percent },
            },
        };
        self
    }
    pub fn y_percent(&mut self, percent:f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_PERCENT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: percent, max: percent },
            },
        };
        self
    }
    pub fn padding_all(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding = Clay_Padding { left: amount, right: amount, top: amount, bottom: amount };
        self
    }
    pub fn padding_top(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.top = amount;
        self
    }
    pub fn padding_bottom(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.bottom = amount;
        self
    }
    pub fn padding_left(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.left = amount;
        self
    }
    pub fn padding_right(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.right = amount;
        self
    }
    pub fn child_gap(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.childGap = amount;
        self
    }
    pub fn direction(&mut self, top_to_bottom: bool) -> &mut Self {
        if top_to_bottom {
            self.decleration.layout.layoutDirection = Clay_LayoutDirection::CLAY_TOP_TO_BOTTOM;
        }
        else {
            self.decleration.layout.layoutDirection = Clay_LayoutDirection::CLAY_LEFT_TO_RIGHT;
        }
        self
    }
    pub fn align_children_x_center(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.x = Clay_LayoutAlignmentX::CLAY_ALIGN_X_CENTER;
        self
    }
    pub fn align_children_x_left(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.x = Clay_LayoutAlignmentX::CLAY_ALIGN_X_LEFT;
        self
    }
    pub fn align_children_x_right(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.x = Clay_LayoutAlignmentX::CLAY_ALIGN_X_RIGHT;
        self
    }
    pub fn align_children_y_center(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.y = Clay_LayoutAlignmentY::CLAY_ALIGN_Y_CENTER;
        self
    }
    pub fn align_children_y_top(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.y = Clay_LayoutAlignmentY::CLAY_ALIGN_Y_TOP;
        self
    }
    pub fn align_children_y_bottom(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.y = Clay_LayoutAlignmentY::CLAY_ALIGN_Y_BOTTOM;
        self
    }
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.decleration.backgroundColor = color.into();
        self
    }
    pub fn radius_all(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius = Clay_CornerRadius {
            topLeft: radius,
            topRight: radius,
            bottomLeft: radius,
            bottomRight: radius,
        };
        self
    }
    pub fn radius_top_left(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.topLeft = radius;
        self
    }
    pub fn radius_top_right(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.topRight = radius;
        self
    }
    pub fn radius_bottom_left(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.bottomLeft = radius;
        self
    }
    pub fn radius_bottom_right(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.bottomLeft = radius;
        self
    }
    pub fn border_color(&mut self, color: Color) -> &mut Self {
        self.decleration.border.color = Clay_Color { r: color.r, g: color.g, b: color.b, a: color.a };
        self
    }
    pub fn border_all(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width = Clay_BorderWidth { left: width, right: width, top: width, bottom: width, betweenChildren: width };
        self
    }
    pub fn border_top(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.top = width;
        self
    }
    pub fn border_left(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.left = width;
        self
    }
    pub fn border_bottom(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.bottom = width;
        self
    }
    pub fn border_right(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.right = width;
        self
    }
    pub fn border_between_children(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.betweenChildren = width;
        self
    }
    pub fn scroll(&mut self, vertical: bool, horizontal: bool) -> &mut Self {
        self.decleration.scroll = Clay_ScrollElementConfig { horizontal, vertical };
        self
    }
    pub fn floating(&mut self) -> &mut Self {
        self.decleration.floating.attachTo = Clay_FloatingAttachToElement::CLAY_ATTACH_TO_PARENT;
        self.decleration.floating = Clay_FloatingElementConfig::default();
        self
    }
    pub fn floating_offset(&mut self, x:f32, y:f32) -> &mut Self{
        self.decleration.floating.offset = Clay_Vector2 { x, y };
        self
    }
    pub fn floating_dimensions(&mut self, width:f32, height:f32) -> &mut Self{
        self.decleration.floating.expand = Clay_Dimensions {width, height};
        self
    }
    pub fn floating_z_index(&mut self, z:i16) -> &mut Self {
        self.decleration.floating.zIndex = z;
        self
    }
    pub fn floating_attach_to_parent_at_top_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_TOP;
        self
    }
    pub fn floating_attach_to_parent_at_center_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_CENTER;
        self
    }
    pub fn floating_attach_to_parent_at_bottom_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_BOTTOM;
        self
    }
    pub fn floating_attach_to_parent_at_top_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_TOP;
        self
    }
    pub fn floating_attach_to_parent_at_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_CENTER;
        self
    }
    pub fn floating_attach_to_parent_at_bottom_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_BOTTOM;
        self
    }
    pub fn floating_attach_to_parent_at_top_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_TOP;
        self
    }
    pub fn floating_attach_to_parent_at_center_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_CENTER;
        self
    }
    pub fn floating_attach_to_parent_at_bottom_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_BOTTOM;
        self
    }
    pub fn floating_attach_element_at_top_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =  Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_TOP;
        self
    }
    pub fn floating_attach_element_at_center_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_CENTER;
        self
    }
    pub fn floating_attach_element_at_bottom_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_BOTTOM;
        self
    }
    pub fn floating_attach_element_at_top_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_TOP;
        self
    }
    pub fn floating_attach_element_at_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_CENTER;
        self
    }
    pub fn floating_attach_element_at_bottom_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_BOTTOM;
        self
    }
    pub fn floating_attach_element_at_top_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_TOP;
        self
    }
    pub fn floating_attach_element_at_center_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_CENTER;
        self
    }
    pub fn floating_attach_element_at_bottom_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element = Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_BOTTOM;
        self
    }
    pub fn floating_pointer_pass_through(&mut self) -> &mut Self {
        self.decleration.floating.pointerCaptureMode = Clay_PointerCaptureMode::CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH;
        self
    }
    pub fn floating_attach_to_element(&mut self, element_id: u32) -> &mut Self {
        self.decleration.floating.parentId = element_id;
        self.decleration.floating.attachTo = Clay_FloatingAttachToElement::CLAY_ATTACH_TO_ELEMENT_WITH_ID;
        self
    }
    pub fn floating_attach_to_root(&mut self) -> &mut Self {
        self.decleration.floating.attachTo = Clay_FloatingAttachToElement::CLAY_ATTACH_TO_ROOT;
        self
    }
    pub fn image<'render_pass, ImageElementData>(&mut self, image: &'render_pass ImageElementData, width: f32, height:f32) -> &mut Self {
        self.decleration.image.imageData = image as *const ImageElementData as *mut c_void;
        self.decleration.image.sourceDimensions = Clay_Dimensions {width, height };
        self
    }
    pub fn custom_element<'render_pass, CustomElementData>(&mut self, custom_element_data: &'render_pass CustomElementData) -> &mut Self{
        self.decleration.custom.customData = custom_element_data as *const CustomElementData as *mut c_void;
        self
    }
    pub fn custom_layout_settings<'render_pass, CustomLayoutSettings>(&mut self, custom_layout_settings: &'render_pass CustomLayoutSettings) -> &mut Self{
        self.decleration.userData = custom_layout_settings as *const CustomLayoutSettings as *mut c_void;
        self
    }
    pub fn parse(&mut self){}
    pub fn end(self) -> Self {
        self
    }
}

impl Into<Clay_ElementDeclaration> for &ElementConfiguration{
    fn into(self) -> Clay_ElementDeclaration {
        self.decleration
    }
}