use crate::bindings::clay::*;

#[derive(Debug, Clone)]
pub struct Id{
    pub id: u32,
    pub offset: u32,
    pub base_id: u32,
    pub string_id: String,
    inner: Clay_ElementId,
}

impl Id {
    /// Creates a clay id using the `label`
    #[inline]
    pub(crate) fn new(label: &str) -> Self {
        let inner = unsafe { Clay__HashString(label.into(), 0, 0) };

        Self {
            id: inner.id,
            offset: inner.offset,
            base_id: inner.baseId,
            string_id: inner.stringId.into(),
            inner,
        }
    }
}

impl Into<Clay_ElementId> for Id{
    fn into(self) -> Clay_ElementId {
        self.inner
    }
}

impl Into<String> for Clay_String{
    fn into(self) -> String {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.chars as *const u8,
                self.length as _,
            ))
        }.to_string()
    }
}

impl From<&str> for Clay_String {
    fn from(value: &str) -> Self {
        Self {
            isStaticallyAllocated: false,
            length: value.len() as _,
            chars: value.as_ptr() as _,
        }
    }
}

impl From<Clay_String> for &str {
    fn from(value: Clay_String) -> Self {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                value.chars as *const u8,
                value.length as _,
            ))
        }
    }
}

impl From<Clay_StringSlice> for &str {
    fn from(value: Clay_StringSlice) -> Self {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                value.chars as *const u8,
                value.length as _,
            ))
        }
    }
}

impl Into<[f32;4]> for Clay_Color{
    fn into(self) -> [f32;4] {
        [self.r,self.g,self.b,self.a]
    }
}

impl Into<(f32, f32, f32, f32)> for Clay_BoundingBox{
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl Into<[f32;4]> for Clay_BoundingBox {
    fn into(self) -> [f32;4] {
        [self.x, self.y, self.width, self.height]
    }
}

impl Into<(f32,f32)> for Clay_Dimensions{
    fn into(self) -> (f32,f32) {
        (self.width, self.height)
    }
}

impl Into<Clay_Padding> for [f32;4] {
    fn into(self) -> Clay_Padding {
        Clay_Padding { 
            left: self[0] as u16, 
            right: self[1] as u16, 
            top: self[2] as u16, 
            bottom: self[3] as u16 
        }
    }
}

impl Into<Clay_CornerRadius> for [f32;4] {
    fn into(self) -> Clay_CornerRadius {
        Clay_CornerRadius { 
            topLeft: self[0], 
            topRight: self[1], 
            bottomLeft: self[2], 
            bottomRight: self[3] 
        }
    }
}


impl Into<Clay_Dimensions> for (f32,f32){
    fn into(self) -> Clay_Dimensions {
        Clay_Dimensions { width: self.0, height: self.1 }
    }
}

impl Into<Clay_Vector2> for (f32,f32){
    fn into(self) -> Clay_Vector2 {
        Clay_Vector2 { x: self.0, y: self.1 }
    }
}
