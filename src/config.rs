use crate::bindings::clay::*;
use crate::type_substitutions::*;

#[allow(dead_code)]
#[derive(Default)]
pub struct ConfigBuilder<'render_pass, ImageElementData: 'render_pass + Default, CustomElementData: 'render_pass + Default>{
    id: Option<Id>,

    sizing_x: Option<Clay_SizingAxis>,
    sizing_y: Option<Clay_SizingAxis>,
    padding: Option<[f32;4]>,
    child_gap: Option<u16>,
    child_alignment_x: Option<Clay_LayoutAlignmentX>,
    child_alignment_y: Option<Clay_LayoutAlignmentY>,
    layout_direction: Option<Clay_LayoutDirection>,

    background_color: Option<[f32;4]>,
    corner_radius: Option<[f32;4]>,

    image: Option<&'render_pass ImageElementData>,
    image_dimension: Option<(u32, u32)>,
    //pub floating: Option<Clay_FloatingElementConfig>,
    custom: Option<&'render_pass CustomElementData>,
    scroll: Option<(bool, bool)>,
    border: Option<([f32;5], [f32;4])>,
}

impl<'render_pass, ImageElementData: Default, CustomElementData: Default> ConfigBuilder<'render_pass, ImageElementData, CustomElementData>{
    pub fn id(&mut self, label: &str) {
        let id = Id::new(label);
        self.id = Some(id);
    }
    pub fn grow_all(&mut self){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        });
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        });
    }
    pub fn x_grow(&mut self){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        });
    }
    pub fn x_grow_min(&mut self, min: f32){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        });
    }
    pub fn x_grow_min_max(&mut self, min: f32, max: f32){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        });
    }
    pub fn y_grow(&mut self){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        });
    }
    pub fn y_grow_min(&mut self, min: f32){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        });
    }
    pub fn y_grow_min_max(&mut self, min: f32, max: f32){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        });
    }
    pub fn x_fit(&mut self){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        });
    }
    pub fn x_fit_min(&mut self, min: f32){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        });
    }
    pub fn x_fit_min_max(&mut self, min: f32,max: f32){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        });
    }
    pub fn y_fit(&mut self){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: 0.0, max: f32::MAX },
            },
        });
    }
    pub fn y_fit_min(&mut self, min: f32){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        });
    }
    pub fn y_fit_min_max(&mut self, min: f32,max: f32){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        });
    }
    pub fn x_fixed(&mut self, size:f32){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: size,
                    max: size,
                },
            },
        });
    }
    pub fn y_fixed(&mut self, size:f32){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: size,
                    max: size,
                },
            },
        });
    }
    pub fn x_percent(&mut self, percent:f32){
        self.sizing_x = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_PERCENT,
            size: Clay_SizingAxis__bindgen_ty_1 { percent },
        });
    }
    pub fn y_percent(&mut self, percent:f32){
        self.sizing_y = Some(Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_PERCENT,
            size: Clay_SizingAxis__bindgen_ty_1 { percent },
        });
    }
    pub fn padding_all(&mut self, amount: f32){
        self.padding = Some([amount, amount, amount, amount]);
    }
    pub fn padding_top(&mut self, amount: f32){
        match self.padding.is_some() {
            true => self.padding.as_mut().unwrap()[0] = amount,
            false => {
                self.padding_all(0.0);
                self.padding.as_mut().unwrap()[0] = amount;
            }
        }
    }
    pub fn padding_bottom(&mut self, amount: f32){
        match self.padding.is_some() {
            true => self.padding.as_mut().unwrap()[1] = amount,
            false => {
                self.padding_all(0.0);
                self.padding.as_mut().unwrap()[1] = amount;
            }
        }
    }
    pub fn padding_left(&mut self, amount: f32){
        match self.padding.is_some() {
            true => self.padding.as_mut().unwrap()[2] = amount,
            false => {
                self.padding_all(0.0);
                self.padding.as_mut().unwrap()[2] = amount;
            }
        }
    }
    pub fn padding_right(&mut self, amount: f32){
        match self.padding.is_some() {
            true => self.padding.as_mut().unwrap()[3] = amount,
            false => {
                self.padding_all(0.0);
                self.padding.as_mut().unwrap()[3] = amount;
            }
        }
    }
    pub fn child_gap(&mut self, amount: u16){
        self.child_gap = Some(amount);
    }
    pub fn direction(&mut self, direction: Clay_LayoutDirection){
        self.layout_direction = Some(direction);
    }
    pub fn align_children_x(&mut self, alignment: Clay_LayoutAlignmentX){
        self.child_alignment_x = Some(alignment);
    }
    pub fn align_children_y(&mut self, alignment: Clay_LayoutAlignmentY){
        self.child_alignment_y = Some(alignment);
    }
    pub fn background_color(&mut self, color: [f32;4]){
        self.background_color = Some(color);
    }
    pub fn radius_all(&mut self, radius: f32){
        let radius = [radius, radius, radius, radius];
        self.corner_radius = Some(radius);
    }
    pub fn radius_top_left(&mut self, radius: f32){
        match self.corner_radius.is_some() {
            true => self.corner_radius.as_mut().unwrap()[0] = radius,
            false => {
                self.radius_all(0.0);
                self.corner_radius.as_mut().unwrap()[0] = radius;
            }
        }
    }
    pub fn radius_top_right(&mut self, radius: f32){
        match self.corner_radius.is_some() {
            true => self.corner_radius.as_mut().unwrap()[1] = radius,
            false => {
                self.radius_all(0.0);
                self.corner_radius.as_mut().unwrap()[1] = radius;
            }
        }
    }
    pub fn radius_bottom_left(&mut self, radius: f32){
        match self.corner_radius.is_some() {
            true => self.corner_radius.as_mut().unwrap()[2] = radius,
            false => {
                self.radius_all(0.0);
                self.corner_radius.as_mut().unwrap()[2] = radius;
            }
        }
    }
    pub fn radius_bottom_right(&mut self, radius: f32){
        match self.corner_radius.is_some() {
            true => self.corner_radius.as_mut().unwrap()[3] = radius,
            false => {
                self.radius_all(0.0);
                self.corner_radius.as_mut().unwrap()[3] = radius;
            }
        }
    }
    pub fn border_all(&mut self, border: f32, color: [f32;4]){
        let border = [border, border, border, border, border];
        self.border = Some((border, color));
    }
    pub fn border_top(&mut self, border: f32, new_color: Option<[f32;4]>){
        match self.border {
            None => {
                self.border = Some(([border, 0.0, 0.0, 0.0, 0.0], 
                    if let Some(new_color) = new_color {
                        new_color
                    }
                    else {
                        [0.0,0.0,0.0,0.0]
                    }
                ))
            }
            #[allow(unused_variables, unused_assignments)]
            Some((mut borders, mut color)) => {
                borders[0] = border; 
                if let Some(new_color) = new_color {
                    color = new_color;
                }
            }
        }
    }
    pub fn border_left(&mut self, border: f32, new_color: Option<[f32;4]>){
        match self.border {
            None => {
                self.border = Some(([0.0, border, 0.0, 0.0, 0.0], 
                    if let Some(new_color) = new_color {
                        new_color
                    }
                    else {
                        [0.0,0.0,0.0,0.0]
                    }
                ))
            }
            #[allow(unused_variables, unused_assignments)]
            Some((mut borders, mut color)) => {
                borders[1] = border; 
                if let Some(new_color) = new_color {
                    color = new_color;
                }
            }
        }
    }
    pub fn border_bottom(&mut self, border: f32, new_color: Option<[f32;4]>){
        match self.border {
            None => {
                self.border = Some(([0.0, 0.0, border, 0.0, 0.0], 
                    if let Some(new_color) = new_color {
                        new_color
                    }
                    else {
                        [0.0,0.0,0.0,0.0]
                    }
                ))
            }
            #[allow(unused_variables, unused_assignments)]
            Some((mut borders, mut color)) => {
                borders[2] = border; 
                if let Some(new_color) = new_color {
                    color = new_color;
                }
            }
        }
    }
    pub fn border_right(&mut self, border: f32, new_color: Option<[f32;4]>){
        match self.border {
            None => {
                self.border = Some(([0.0, 0.0, 0.0, border, 0.0], 
                    if let Some(new_color) = new_color {
                        new_color
                    }
                    else {
                        [0.0,0.0,0.0,0.0]
                    }
                ))
            }
            #[allow(unused_variables, unused_assignments)]
            Some((mut borders, mut color)) => {
                borders[3] = border; 
                if let Some(new_color) = new_color {
                    color = new_color;
                }
            }
        }
    }
    pub fn border_between_children(&mut self, border: f32, new_color: Option<[f32;4]>){
        match self.border {
            None => {
                self.border = Some(([0.0, 0.0, 0.0, 0.0, border], 
                    if let Some(new_color) = new_color {
                        new_color
                    }
                    else {
                        [0.0,0.0,0.0,0.0]
                    }
                ))
            }
            #[allow(unused_variables, unused_assignments)]
            Some((mut borders, mut color)) => {
                borders[4] = border; 
                if let Some(new_color) = new_color {
                    color = new_color;
                }
            }
        }
    }
    pub fn scroll(&mut self, vertical: bool, horizontal: bool){
        self.scroll = Some((vertical, horizontal));
    }
    pub fn image_data(&mut self, data: &'render_pass ImageElementData){
        self.image = Some(data);
    }
    pub fn image_size(&mut self, size: (u32, u32)){
        self.image_dimension = Some(size);
    }
}

impl<'render_pass, ImageElementData: Default, CustomElementData: Default> Into<Clay_ElementDeclaration> for ConfigBuilder<'render_pass, ImageElementData, CustomElementData>{
    fn into(self) -> Clay_ElementDeclaration {
        let mut element_declaration = Clay_ElementDeclaration::default();

        if let Some(id) = self.id {
            element_declaration.id = id.into();
        }
        
        if self.sizing_x.is_some() ||
            self.sizing_y.is_some() ||
            self.padding.is_some() ||
            self.child_gap.is_some() ||
            self.child_alignment_x.is_some() ||
            self.child_alignment_y.is_some() ||
            self.layout_direction.is_some()
        {
            let mut layout = Clay_LayoutConfig::default();

            if self.sizing_x.is_some() ||
                self.sizing_y.is_some() {
                
                let mut clay_sizing = Clay_Sizing::default();

                if let Some(sizing_x) = self.sizing_x {
                    clay_sizing.width = sizing_x;
                }

                if let Some(sizing_y) = self.sizing_y {
                    clay_sizing.height = sizing_y;
                }

                layout.sizing = clay_sizing;
            }

            if let Some(padding) = self.padding {
                layout.padding = padding.into();
            }

            if let Some(child_gap) = self.child_gap {
                layout.childGap = child_gap;
            }

            if let Some(child_alignment_x) = self.child_alignment_x {
                layout.childAlignment.x = child_alignment_x as _;
            }

            if let Some(child_alignment_y) = self.child_alignment_y{
                layout.childAlignment.y = child_alignment_y as _;
            }

            if let Some(layout_direction) = self.layout_direction {
                layout.layoutDirection = layout_direction as _;
            }

            element_declaration.layout = layout;
        }

        if let Some(bg_color) = self.background_color {
            element_declaration.backgroundColor = Clay_Color{
                r: bg_color[0],
                g: bg_color[1],
                b: bg_color[2],
                a: bg_color[3],
            };
        }

        if let Some(corner_radii) = self.corner_radius {
            element_declaration.cornerRadius = corner_radii.into();
        }

        if let Some((border, color)) = self.border {
            element_declaration.border = Clay_BorderElementConfig {
                color: Clay_Color { r: color[0], g: color[1], b: color[2], a: color[3] },
                width: Clay_BorderWidth {
                    top: border[0] as u16,
                    left: border[1] as u16,
                    bottom: border[2] as u16,
                    right: border[3] as u16,
                    betweenChildren: border[4] as u16
                }
            }
        }

        if let Some(scroll) = self.scroll {
            element_declaration.scroll = Clay_ScrollElementConfig {vertical: scroll.0, horizontal: scroll.1};
        }

        element_declaration
    }
}
