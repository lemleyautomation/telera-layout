use core::mem::MaybeUninit;
use std::os::raw::c_void;

use crate::bindings::*;

/// Builds a `Clay_ElementDeclaration` for a single element through chained setters,
/// then feeds it to [`LayoutEngine::configure_element`](crate::LayoutEngine::configure_element).
///
/// Each setter writes one field (or one axis / side) and returns `&mut Self`, so
/// setters accumulate regardless of order; call [`end`](Self::end) to take the
/// finished value out of the `&mut` chain.
///
/// ```
/// use telera_layout::{Color, ElementConfiguration};
///
/// let panel = ElementConfiguration::new()
///     .id("panel")
///     .grow_all()
///     .padding_all(8)
///     .child_gap(4)
///     .color(Color::rgb(30, 30, 40))
///     .radius_all(6.0)
///     .end();
/// ```
#[derive(Default, Clone, Copy)]
pub struct ElementConfiguration {
    decleration: Clay_ElementDeclaration,
}

impl ElementConfiguration {
    /// A configuration with every field zeroed, which is also clay's set of defaults
    /// (fit sizing, no padding, left-to-right, transparent, not floating).
    pub const fn new() -> Self {
        let inner = MaybeUninit::<Clay_ElementDeclaration>::zeroed();
        ElementConfiguration {
            decleration: unsafe { inner.assume_init() },
        }
    }

    /// A copy of `other`, as a starting point for a variant configuration.
    pub fn with(other: ElementConfiguration) -> Self {
        ElementConfiguration {
            decleration: other.decleration,
        }
    }

    /// Sets the element id by hashing `label` (clay's `CLAY_ID`), for later use with
    /// [`LayoutEngine::pointer_over`](crate::LayoutEngine::pointer_over),
    /// [`bounding_box`](crate::LayoutEngine::bounding_box) and floating attachment.
    ///
    /// ```
    /// use telera_layout::ElementConfiguration;
    ///
    /// let cfg = ElementConfiguration::new().id("sidebar").end();
    /// ```
    pub fn id(&mut self, label: &str) -> &mut Self {
        self.id_indexed(label, 0)
    }

    /// Sets the element id by hashing `label` together with `index` (clay's
    /// `CLAY_IDI`), so unique ids can be generated inside a loop without building
    /// dynamic strings.
    ///
    /// ```
    /// use telera_layout::ElementConfiguration;
    ///
    /// for i in 0..3 {
    ///     let _row = ElementConfiguration::new().id_indexed("row", i).end();
    /// }
    /// ```
    pub fn id_indexed(&mut self, label: &str, index: u32) -> &mut Self {
        self.decleration.id = unsafe {
            Clay__HashString(
                Clay_String {
                    isStaticallyAllocated: true,
                    length: label.len() as i32,
                    chars: label.as_ptr() as *const _,
                },
                index,
                0,
            )
        };
        self
    }
    /// Grows both axes to fill the remaining space in the parent, shared with other
    /// growing siblings.
    pub const fn grow_all(&mut self) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self
    }
    /// Grows the width to fill the remaining horizontal space in the parent.
    pub const fn x_grow(&mut self) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self
    }
    /// Grows the width to fill remaining horizontal space, never shrinking below `min`.
    pub const fn x_grow_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    /// Grows the width to fill remaining horizontal space, clamped to `min..=max`.
    pub const fn x_grow_min_max(&mut self, min: f32, max: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    /// Grows the height to fill the remaining vertical space in the parent.
    pub const fn y_grow(&mut self) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self
    }
    /// Grows the height to fill remaining vertical space, never shrinking below `min`.
    pub const fn y_grow_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    /// Grows the height to fill remaining vertical space, clamped to `min..=max`.
    pub const fn y_grow_min_max(&mut self, min: f32, max: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_GROW,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    /// Sizes both axes to fit their content.
    pub const fn fit_all(&mut self) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self
    }
    /// Sizes the width to fit its content.
    pub const fn x_fit(&mut self) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self
    }
    /// Sizes the width to fit its content, but never narrower than `min`.
    pub const fn x_fit_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    /// Sizes the width to fit its content, clamped to `min..=max`.
    pub const fn x_fit_min_max(&mut self, min: f32, max: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    /// Sizes the height to fit its content.
    pub const fn y_fit(&mut self) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: 0.0,
                    max: f32::MAX,
                },
            },
        };
        self
    }
    /// Sizes the height to fit its content, but never shorter than `min`.
    pub const fn y_fit_min(&mut self, min: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max: f32::MAX },
            },
        };
        self
    }
    /// Sizes the height to fit its content, clamped to `min..=max`.
    pub const fn y_fit_min_max(&mut self, min: f32, max: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min, max },
            },
        };
        self
    }
    /// Fixes the element to exactly `x` pixels wide and `y` pixels tall.
    pub const fn fixed(&mut self, x: f32, y: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: x, max: x },
            },
        };
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax { min: y, max: y },
            },
        };
        self
    }
    /// Fixes the element to a `size` × `size` pixel square.
    pub const fn fixed_square(&mut self, size: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: size,
                    max: size,
                },
            },
        };
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: size,
                    max: size,
                },
            },
        };
        self
    }
    /// Fixes the width to exactly `size` pixels.
    pub const fn x_fixed(&mut self, size: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: size,
                    max: size,
                },
            },
        };
        self
    }
    /// Fixes the height to exactly `size` pixels.
    pub const fn y_fixed(&mut self, size: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_FIXED,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: size,
                    max: size,
                },
            },
        };
        self
    }
    /// Sizes the width to `percent` (0.0..=1.0) of the parent's inner width.
    pub const fn x_percent(&mut self, percent: f32) -> &mut Self {
        self.decleration.layout.sizing.width = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_PERCENT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: percent,
                    max: percent,
                },
            },
        };
        self
    }
    /// Sizes the height to `percent` (0.0..=1.0) of the parent's inner height.
    pub const fn y_percent(&mut self, percent: f32) -> &mut Self {
        self.decleration.layout.sizing.height = Clay_SizingAxis {
            type_: Clay__SizingType::CLAY__SIZING_TYPE_PERCENT,
            size: Clay_SizingAxis__bindgen_ty_1 {
                minMax: Clay_SizingMinMax {
                    min: percent,
                    max: percent,
                },
            },
        };
        self
    }
    /// Sets the same padding, in pixels, on all four sides.
    pub const fn padding_all(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding = Clay_Padding {
            left: amount,
            right: amount,
            top: amount,
            bottom: amount,
        };
        self
    }
    /// Sets the top padding in pixels.
    pub const fn padding_top(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.top = amount;
        self
    }
    /// Sets the bottom padding in pixels.
    pub const fn padding_bottom(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.bottom = amount;
        self
    }
    /// Sets the left padding in pixels.
    pub const fn padding_left(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.left = amount;
        self
    }
    /// Sets the right padding in pixels.
    pub const fn padding_right(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.padding.right = amount;
        self
    }
    /// Sets the gap, in pixels, inserted between adjacent children along the layout axis.
    pub const fn child_gap(&mut self, amount: u16) -> &mut Self {
        self.decleration.layout.childGap = amount;
        self
    }
    /// Lays children out top-to-bottom when `top_to_bottom` is `true`, otherwise
    /// left-to-right.
    pub const fn direction(&mut self, top_to_bottom: bool) -> &mut Self {
        if top_to_bottom {
            self.decleration.layout.layoutDirection = Clay_LayoutDirection::CLAY_TOP_TO_BOTTOM;
        } else {
            self.decleration.layout.layoutDirection = Clay_LayoutDirection::CLAY_LEFT_TO_RIGHT;
        }
        self
    }
    /// Centers children horizontally within this element.
    pub const fn align_children_x_center(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.x = Clay_LayoutAlignmentX::CLAY_ALIGN_X_CENTER;
        self
    }
    /// Aligns children to this element's left edge (clay's default).
    pub const fn align_children_x_left(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.x = Clay_LayoutAlignmentX::CLAY_ALIGN_X_LEFT;
        self
    }
    /// Aligns children to this element's right edge.
    pub const fn align_children_x_right(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.x = Clay_LayoutAlignmentX::CLAY_ALIGN_X_RIGHT;
        self
    }
    /// Centers children vertically within this element.
    pub const fn align_children_y_center(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.y = Clay_LayoutAlignmentY::CLAY_ALIGN_Y_CENTER;
        self
    }
    /// Aligns children to this element's top edge (clay's default).
    pub const fn align_children_y_top(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.y = Clay_LayoutAlignmentY::CLAY_ALIGN_Y_TOP;
        self
    }
    /// Aligns children to this element's bottom edge.
    pub const fn align_children_y_bottom(&mut self) -> &mut Self {
        self.decleration.layout.childAlignment.y = Clay_LayoutAlignmentY::CLAY_ALIGN_Y_BOTTOM;
        self
    }
    /// Sets the background color. With no other element config this produces a
    /// `RECTANGLE` render command; otherwise it is passed to the `IMAGE` / `CUSTOM`
    /// command.
    pub const fn color(&mut self, color: Color) -> &mut Self {
        self.decleration.backgroundColor = color.to_clay();
        self
    }
    /// Sets the target aspect ratio (final width / final height) for this element
    /// (clay's `.aspectRatio`). Primarily useful together with [`Self::image`].
    pub const fn aspect_ratio(&mut self, ratio: f32) -> &mut Self {
        self.decleration.aspectRatio.aspectRatio = ratio;
        self
    }
    /// Sets the same corner radius, in pixels, on all four corners.
    pub const fn radius_all(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius = Clay_CornerRadius {
            topLeft: radius,
            topRight: radius,
            bottomLeft: radius,
            bottomRight: radius,
        };
        self
    }
    /// Sets the top-left corner radius in pixels.
    pub const fn radius_top_left(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.topLeft = radius;
        self
    }
    /// Sets the top-right corner radius in pixels.
    pub const fn radius_top_right(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.topRight = radius;
        self
    }
    /// Sets the bottom-left corner radius in pixels.
    pub const fn radius_bottom_left(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.bottomLeft = radius;
        self
    }
    /// Sets the bottom-right corner radius in pixels.
    pub const fn radius_bottom_right(&mut self, radius: f32) -> &mut Self {
        self.decleration.cornerRadius.bottomRight = radius;
        self
    }
    /// Sets the color used for every border side with a non-zero width.
    pub const fn border_color(&mut self, color: Color) -> &mut Self {
        self.decleration.border.color = Clay_Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
        self
    }
    /// Sets the same border width, in pixels, on all four sides and between children.
    pub const fn border_all(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width = Clay_BorderWidth {
            left: width,
            right: width,
            top: width,
            bottom: width,
            betweenChildren: width,
        };
        self
    }
    /// Sets the top border width in pixels.
    pub const fn border_top(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.top = width;
        self
    }
    /// Sets the left border width in pixels.
    pub const fn border_left(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.left = width;
        self
    }
    /// Sets the bottom border width in pixels.
    pub const fn border_bottom(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.bottom = width;
        self
    }
    /// Sets the right border width in pixels.
    pub const fn border_right(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.right = width;
        self
    }
    /// Sets the width, in pixels, of the border drawn between adjacent children.
    pub const fn border_between_children(&mut self, width: u16) -> &mut Self {
        self.decleration.border.width.betweenChildren = width;
        self
    }
    /// Clip (and thereby allow scrolling of) overflowing content on the given axes,
    /// matching clay's `.clip`. Set at least one axis to `true` to generate scissor
    /// render commands.
    pub const fn clip(&mut self, vertical: bool, horizontal: bool) -> &mut Self {
        self.decleration.clip.vertical = vertical;
        self.decleration.clip.horizontal = horizontal;
        self
    }
    /// Offsets the position of all child elements, used to implement scrolling of a
    /// clipped container (clay's `.clip.childOffset`).
    pub const fn clip_child_offset(&mut self, x: f32, y: f32) -> &mut Self {
        self.decleration.clip.childOffset = Clay_Vector2 { x, y };
        self
    }
    /// Activates floating mode by attaching this element to its parent (clay's
    /// `.floating.attachTo = CLAY_ATTACH_TO_PARENT`). All other `floating_*` builders
    /// only take effect once floating has been activated by this method or one of the
    /// `floating_attach_to_*` methods. Leaves every other floating field at clay's
    /// defaults so it can be refined by the other builders in any order.
    ///
    /// ```
    /// use telera_layout::ElementConfiguration;
    ///
    /// let tooltip = ElementConfiguration::new()
    ///     .floating()
    ///     .floating_attach_element_at_top_center()
    ///     .floating_attach_to_parent_at_bottom_center()
    ///     .floating_offset(0.0, 4.0)
    ///     .floating_z_index(10)
    ///     .end();
    /// ```
    pub const fn floating(&mut self) -> &mut Self {
        self.decleration.floating.attachTo = Clay_FloatingAttachToElement::CLAY_ATTACH_TO_PARENT;
        self
    }
    /// Clip this floating element to the same rectangle as the element it is attached
    /// to (clay's `.floating.clipTo = CLAY_CLIP_TO_ATTACHED_PARENT`).
    pub const fn floating_clip_to_attached_parent(&mut self) -> &mut Self {
        self.decleration.floating.clipTo = Clay_FloatingClipToElement::CLAY_CLIP_TO_ATTACHED_PARENT;
        self
    }
    /// Do not inherit clipping from the attached element (clay's default,
    /// `.floating.clipTo = CLAY_CLIP_TO_NONE`).
    pub const fn floating_no_clip(&mut self) -> &mut Self {
        self.decleration.floating.clipTo = Clay_FloatingClipToElement::CLAY_CLIP_TO_NONE;
        self
    }
    /// Capture pointer events, preventing hover/click from passing through to elements
    /// underneath this floating element (clay's default,
    /// `.floating.pointerCaptureMode = CLAY_POINTER_CAPTURE_MODE_CAPTURE`).
    pub const fn floating_pointer_capture(&mut self) -> &mut Self {
        self.decleration.floating.pointerCaptureMode =
            Clay_PointerCaptureMode::CLAY_POINTER_CAPTURE_MODE_CAPTURE;
        self
    }
    /// Offsets the floating element by `(x, y)` pixels from its resolved attach point.
    pub const fn floating_offset(&mut self, x: f32, y: f32) -> &mut Self {
        self.decleration.floating.offset = Clay_Vector2 { x, y };
        self
    }
    /// Expands the floating element's outer bounding box by `(width, height)` pixels
    /// without affecting its children (clay's `.floating.expand`).
    pub const fn floating_dimensions(&mut self, width: f32, height: f32) -> &mut Self {
        self.decleration.floating.expand = Clay_Dimensions { width, height };
        self
    }
    /// Sets the z order of this floating element and its children; higher draws on top.
    pub const fn floating_z_index(&mut self, z: i16) -> &mut Self {
        self.decleration.floating.zIndex = z;
        self
    }
    /// Anchors to the parent's top-left corner.
    pub const fn floating_attach_to_parent_at_top_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_TOP;
        self
    }
    /// Anchors to the middle of the parent's left edge.
    pub const fn floating_attach_to_parent_at_center_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_CENTER;
        self
    }
    /// Anchors to the parent's bottom-left corner.
    pub const fn floating_attach_to_parent_at_bottom_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_BOTTOM;
        self
    }
    /// Anchors to the middle of the parent's top edge.
    pub const fn floating_attach_to_parent_at_top_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_TOP;
        self
    }
    /// Anchors to the parent's center.
    pub const fn floating_attach_to_parent_at_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_CENTER;
        self
    }
    /// Anchors to the middle of the parent's bottom edge.
    pub const fn floating_attach_to_parent_at_bottom_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_BOTTOM;
        self
    }
    /// Anchors to the parent's top-right corner.
    pub const fn floating_attach_to_parent_at_top_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_TOP;
        self
    }
    /// Anchors to the middle of the parent's right edge.
    pub const fn floating_attach_to_parent_at_center_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_CENTER;
        self
    }
    /// Anchors to the parent's bottom-right corner.
    pub const fn floating_attach_to_parent_at_bottom_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.parent =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_BOTTOM;
        self
    }
    /// Uses this element's own top-left corner as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_top_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_TOP;
        self
    }
    /// Uses the middle of this element's left edge as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_center_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_CENTER;
        self
    }
    /// Uses this element's own bottom-left corner as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_bottom_left(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_LEFT_BOTTOM;
        self
    }
    /// Uses the middle of this element's top edge as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_top_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_TOP;
        self
    }
    /// Uses this element's center as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_CENTER;
        self
    }
    /// Uses the middle of this element's bottom edge as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_bottom_center(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_BOTTOM;
        self
    }
    /// Uses this element's own top-right corner as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_top_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_TOP;
        self
    }
    /// Uses the middle of this element's right edge as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_center_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_CENTER;
        self
    }
    /// Uses this element's own bottom-right corner as the point placed on the parent anchor.
    pub const fn floating_attach_element_at_bottom_right(&mut self) -> &mut Self {
        self.decleration.floating.attachPoints.element =
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_BOTTOM;
        self
    }
    /// Lets hover/click events pass through this floating element to whatever is
    /// underneath (clay's `CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH`).
    pub const fn floating_pointer_pass_through(&mut self) -> &mut Self {
        self.decleration.floating.pointerCaptureMode =
            Clay_PointerCaptureMode::CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH;
        self
    }
    /// Activates floating mode and attaches this element to the element whose id is
    /// `element_id` (from [`LayoutEngine::get_element_id`](crate::LayoutEngine::get_element_id)
    /// or [`Self::id`]), rather than to the parent.
    pub const fn floating_attach_to_element(&mut self, element_id: u32) -> &mut Self {
        self.decleration.floating.parentId = element_id;
        self.decleration.floating.attachTo =
            Clay_FloatingAttachToElement::CLAY_ATTACH_TO_ELEMENT_WITH_ID;
        self
    }
    /// Activates floating mode and attaches this element to the layout root, giving
    /// behavior similar to absolute positioning.
    pub const fn floating_attach_to_root(&mut self) -> &mut Self {
        self.decleration.floating.attachTo = Clay_FloatingAttachToElement::CLAY_ATTACH_TO_ROOT;
        self
    }

    /// Marks this element as an image and stores the address of `image` for clay to
    /// pass through to the `IMAGE` render command as
    /// [`Image::data`](crate::Image). `image` must outlive every layout pass the
    /// configuration is used in.
    ///
    /// ```
    /// use telera_layout::ElementConfiguration;
    ///
    /// struct Texture(u32);
    /// let logo = Texture(7);
    /// let cfg = ElementConfiguration::new().image(&logo).aspect_ratio(2.0).end();
    /// ```
    pub const fn image<ImageElementData>(&mut self, image: &ImageElementData) -> &mut Self {
        self.decleration.image.imageData = image as *const ImageElementData as *mut c_void;
        self
    }

    /// Marks this element as custom and stores the address of `custom_element_data`
    /// for clay to pass through to the `CUSTOM` render command as
    /// [`Custom::data`](crate::Custom). The referent must outlive every layout pass
    /// the configuration is used in.
    pub const fn custom_element<CustomElementData>(
        &mut self,
        custom_element_data: &CustomElementData,
    ) -> &mut Self {
        self.decleration.custom.customData =
            custom_element_data as *const CustomElementData as *mut c_void;
        self
    }

    /// Stores the address of `custom_layout_settings` in the element's `userData`, so
    /// clay passes it through untouched to every render command produced by this
    /// element as
    /// [`custom_layout_settings`](crate::Rectangle::custom_layout_settings). The
    /// referent must outlive every layout pass the configuration is used in.
    pub const fn custom_layout_settings<CustomLayoutSettings>(
        &mut self,
        custom_layout_settings: &CustomLayoutSettings,
    ) -> &mut Self {
        self.decleration.userData =
            custom_layout_settings as *const CustomLayoutSettings as *mut c_void;
        self
    }

    /// Ends the chained-`&mut` builder, returning the finished configuration by value.
    pub const fn end(self) -> Self {
        self
    }
}

impl From<&ElementConfiguration> for Clay_ElementDeclaration {
    /// Copies out the accumulated `Clay_ElementDeclaration`.
    fn from(value: &ElementConfiguration) -> Self {
        value.decleration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl(config: &ElementConfiguration) -> Clay_ElementDeclaration {
        config.into()
    }

    /// Every builder that writes into a shared sub-struct (`floating`, `clip`,
    /// `padding`, `border.width`, `cornerRadius`) must only touch its own field, so
    /// that chaining them in any order accumulates rather than clobbers.
    #[test]
    fn builders_do_not_clobber_each_other() {
        // floating: the activator is called *after* every refinement and must keep them.
        let mut c = ElementConfiguration::new();
        c.floating_offset(3.0, 4.0)
            .floating_dimensions(10.0, 20.0)
            .floating_z_index(7)
            .floating_attach_element_at_center()
            .floating_attach_to_parent_at_bottom_right()
            .floating_pointer_pass_through()
            .floating_clip_to_attached_parent()
            .floating();
        let d = decl(&c);
        assert_eq!(d.floating.offset.x, 3.0);
        assert_eq!(d.floating.offset.y, 4.0);
        assert_eq!(d.floating.expand.width, 10.0);
        assert_eq!(d.floating.expand.height, 20.0);
        assert_eq!(d.floating.zIndex, 7);
        assert_eq!(
            d.floating.attachPoints.element,
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_CENTER_CENTER
        );
        assert_eq!(
            d.floating.attachPoints.parent,
            Clay_FloatingAttachPointType::CLAY_ATTACH_POINT_RIGHT_BOTTOM
        );
        assert_eq!(
            d.floating.pointerCaptureMode,
            Clay_PointerCaptureMode::CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH
        );
        assert_eq!(
            d.floating.clipTo,
            Clay_FloatingClipToElement::CLAY_CLIP_TO_ATTACHED_PARENT
        );
        assert_eq!(
            d.floating.attachTo,
            Clay_FloatingAttachToElement::CLAY_ATTACH_TO_PARENT
        );

        // clip: the two axes and the child offset are independent.
        let mut c = ElementConfiguration::new();
        c.clip_child_offset(5.0, 6.0).clip(true, false);
        let d = decl(&c);
        assert!(d.clip.vertical);
        assert!(!d.clip.horizontal);
        assert_eq!(d.clip.childOffset.x, 5.0);
        assert_eq!(d.clip.childOffset.y, 6.0);

        // padding: per-side setters after padding_all keep the untouched sides.
        let mut c = ElementConfiguration::new();
        c.padding_all(2).padding_left(9);
        let d = decl(&c);
        assert_eq!(d.layout.padding.left, 9);
        assert_eq!(d.layout.padding.right, 2);
        assert_eq!(d.layout.padding.top, 2);
        assert_eq!(d.layout.padding.bottom, 2);

        // border width + corner radius: same story.
        let mut c = ElementConfiguration::new();
        c.border_all(1).border_top(4).radius_all(8.0).radius_bottom_right(1.0);
        let d = decl(&c);
        assert_eq!(d.border.width.top, 4);
        assert_eq!(d.border.width.left, 1);
        assert_eq!(d.border.width.betweenChildren, 1);
        assert_eq!(d.cornerRadius.topLeft, 8.0);
        assert_eq!(d.cornerRadius.bottomRight, 1.0);

        // independent top-level fields never interfere.
        let mut c = ElementConfiguration::new();
        c.color(Color::rgb(1, 2, 3))
            .aspect_ratio(1.5)
            .child_gap(4)
            .direction(true)
            .align_children_x_center()
            .align_children_y_bottom();
        let d = decl(&c);
        assert_eq!(d.backgroundColor.r, 1.0);
        assert_eq!(d.aspectRatio.aspectRatio, 1.5);
        assert_eq!(d.layout.childGap, 4);
        assert_eq!(
            d.layout.layoutDirection,
            Clay_LayoutDirection::CLAY_TOP_TO_BOTTOM
        );
        assert_eq!(
            d.layout.childAlignment.x,
            Clay_LayoutAlignmentX::CLAY_ALIGN_X_CENTER
        );
        assert_eq!(
            d.layout.childAlignment.y,
            Clay_LayoutAlignmentY::CLAY_ALIGN_Y_BOTTOM
        );
    }

    #[test]
    fn sizing_axes_are_independent() {
        let mut c = ElementConfiguration::new();
        c.x_fixed(100.0).y_percent(0.5);
        let d = decl(&c);
        assert_eq!(
            d.layout.sizing.width.type_,
            Clay__SizingType::CLAY__SIZING_TYPE_FIXED
        );
        assert_eq!(
            d.layout.sizing.height.type_,
            Clay__SizingType::CLAY__SIZING_TYPE_PERCENT
        );
        unsafe {
            assert_eq!(d.layout.sizing.width.size.minMax.min, 100.0);
            assert_eq!(d.layout.sizing.height.size.percent, 0.5);
        }
    }

    #[test]
    fn id_indexed_differs_from_plain_id() {
        let mut a = ElementConfiguration::new();
        a.id("row");
        let mut b = ElementConfiguration::new();
        b.id_indexed("row", 1);
        assert_ne!(decl(&a).id.id, decl(&b).id.id);
        assert_eq!(decl(&b).id.offset, 1);
    }
}
