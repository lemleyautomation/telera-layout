mod bindings;
use bindings::*;
pub use bindings::{
    Color, Vec2, RenderCommand,
    Rectangle, Border,
    Image, Custom,
    CornerRadii, BorderWidth,
    BoundingBox
};

mod text_configuration;
use text_configuration::*;
pub use text_configuration::TextConfig;
pub use text_configuration::MeasureText;

mod element_configuration;
pub use element_configuration::ElementConfiguration;

use std::{
    fmt::Debug, marker::PhantomData, os::raw::c_void,
};

unsafe extern "C" fn error_handler(error_data: Clay_ErrorData) {
    unsafe {
        let text = core::str::from_utf8_unchecked(
            core::slice::from_raw_parts(
            error_data.errorText.chars as *const u8,
            error_data.errorText.length as _,
            )
        );

        println!("Clay Error: (type: {:?}) {:?}", error_data.errorType, text);
    }
}


pub struct LayoutEngine<ImageElementData: Debug, CustomElementData: Debug, CustomLayoutSettings>{
    _memory: Vec<u8>,
    context: *mut Clay_Context,
    _phantom: PhantomData<(CustomElementData, ImageElementData, CustomLayoutSettings)>,
    dangling_element_count: u32,
}


impl<ImageElementData: Debug, CustomElementData: Debug, CustomLayoutSettings> LayoutEngine<ImageElementData, CustomElementData, CustomLayoutSettings> {
    pub fn new(dimensions: (f32,f32)) -> Self{
        let memory_size = unsafe { Clay_MinMemorySize() as usize };
        let memory = vec![0; memory_size];
        let context;

        unsafe {
            let arena =
                Clay_CreateArenaWithCapacityAndMemory(memory_size, memory.as_ptr() as *mut c_void);

            context = Clay_Initialize(
                arena,
                Clay_Dimensions { width: dimensions.0, height: dimensions.1 },
                Clay_ErrorHandler {
                    errorHandlerFunction: Some(error_handler),
                    userData: std::ptr::null_mut(),
                },
            );
        }

        Self {
            _memory: memory,
            context,
            _phantom: PhantomData{},
            dangling_element_count: 0,
        }
    }

    fn dangle(&mut self){
        self.dangling_element_count += 1;
    }

    fn undangle(&mut self){
        if let Some(dangling_element_count) = self.dangling_element_count.checked_sub(1) {
            self.dangling_element_count = dangling_element_count;
        }
    }

    pub fn set_debug_mode(&self, enable: bool) {
        unsafe {
            Clay_SetDebugModeEnabled(enable);
        }
    }

    pub fn set_layout_dimensions(&self, width: f32, height: f32) {
        unsafe {
            Clay_SetLayoutDimensions(Clay_Dimensions { width, height });
        }
    }

    pub fn begin_layout(&mut self){
        unsafe { 
            Clay_BeginLayout();
            Clay_SetCurrentContext(self.context);
        };
    }

    pub fn end_layout<'render_pass, TextRenderer: MeasureText>(&mut self, text_renderer: &mut TextRenderer) -> Vec<RenderCommand::<'render_pass, ImageElementData, CustomElementData, CustomLayoutSettings>> {
        
        let ptr: *mut TextRenderer = text_renderer;
        let ptr = ptr as *mut c_void;
        unsafe {
            Clay_SetMeasureTextFunction(
                Some(measure_text_c_callback::<TextRenderer>), 
                ptr
            );
        }
        
        assert!(
            self.dangling_element_count == 0 && self.dangling_element_count%2 == 0,
            "All elements must have a Configuration!"
        );

        let array = unsafe {
            let render_commands = Clay_EndLayout();
            Clay_SetMeasureTextFunction(None, std::ptr::null::<c_void>() as _);
            core::slice::from_raw_parts(render_commands.internalArray, render_commands.length as usize)
        };
        
        array.iter().map(|command| {
            match command.commandType {
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_NONE => RenderCommand::None,
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_RECTANGLE => RenderCommand::Rectangle(command.into()),
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_BORDER => RenderCommand::Border(command.into()),
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_TEXT => RenderCommand::Text(command.into()),
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_IMAGE => RenderCommand::Image(command.into()),
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_CUSTOM => RenderCommand::Custom(command.into()),
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_SCISSOR_START => RenderCommand::ScissorStart(command.into()),
                Clay_RenderCommandType::CLAY_RENDER_COMMAND_TYPE_SCISSOR_END => RenderCommand::ScissorEnd
            }
        }).collect::<Vec<RenderCommand::<ImageElementData, CustomElementData, CustomLayoutSettings>>>()
    }

    pub fn open_element(&mut self){
        self.dangle();
        unsafe {
            Clay__OpenElement();
        }
    }

    pub fn close_element(&mut self){
        assert!(
            self.dangling_element_count == 0 && self.dangling_element_count%2 == 0,
            "All elements must have a Configuration!"
        );

        unsafe {
            Clay__CloseElement();
        }
    }

    pub fn configure_element<'render_pass>(&mut self, config: &ElementConfiguration) -> u32 {
        self.undangle();
        unsafe {
            Clay__ConfigureOpenElement(config.into());
            Clay_GetOpenElementId()
        }
    }
    
    pub fn add_text_element<'render_pass, TextRenderer: MeasureText>(&mut self, content: &'render_pass str, config: &'render_pass TextConfig, statically_allicated: bool, text_renderer: &mut TextRenderer) {
        
        let ptr: *mut TextRenderer = text_renderer;
        let ptr = ptr as *mut c_void;
        unsafe {
            Clay_SetMeasureTextFunction(
                Some(measure_text_c_callback::<TextRenderer>), 
                ptr
            );
        }
        
        assert!(
            self.dangling_element_count == 0 && self.dangling_element_count%2 == 0,
            "All elements must have a Configuration!"
        );

        let text_config = unsafe { Clay__StoreTextElementConfig(config.into()) };
        unsafe { 
            Clay__OpenTextElement( 
                Clay_String { 
                    isStaticallyAllocated: statically_allicated, 
                    length: content.len() as i32, 
                    chars: content.as_ptr() as *mut _
                }, 
                text_config 
            ) 
        };

        unsafe {
            Clay_SetMeasureTextFunction(None, std::ptr::null::<c_void>() as _);
        }
    }

    pub fn pointer_state(&self, x: f32, y: f32, is_down: bool) {
        unsafe {
            Clay_SetPointerState(Clay_Vector2 { x, y }, is_down);
        }
    }

    pub fn update_scroll_containers(
        &self,
        drag_scrolling_enabled: bool,
        delta_x: f32,
        delta_y: f32,
        delta_time: f32,
    ) {
        unsafe {
            Clay_UpdateScrollContainers(drag_scrolling_enabled, Clay_Vector2 { x: delta_x, y: delta_y }, delta_time);
        }
    }

    pub fn get_scroll_offset(&self) -> Clay_Vector2{
        unsafe {
            return Clay_GetScrollOffset()
        }
    }

    pub fn get_element_id(&self, id: &str) -> Clay_ElementId {
        let id = unsafe {
            Clay_GetElementId(
                Clay_String { 
                    isStaticallyAllocated: false,
                    length: id.len() as i32, 
                    chars: id.as_ptr() as *const i8
                }
            )
        };

        id
    }

    pub fn scroll_container_data(&self, id: Clay_ElementId) -> Option<Clay_ScrollContainerData> {
        unsafe {
            Clay_SetCurrentContext(self.context);
            let scroll_container_data = Clay_GetScrollContainerData(id);

            if scroll_container_data.found {
                Some(scroll_container_data)
            } else {
                None
            }
        }
    }

    /// Returns if the current element you are creating is hovered
    pub fn hovered(&self) -> bool {
        unsafe { Clay_Hovered() }
    }

    // pub fn on_click<callback: Fn()>(&mut self, callback_function: callback){
    //     unsafe {
    //         Clay_OnHover(Some(LayoutEngine::<(),(),()>::call_back_handler), 0);
    //     }
    // }

    pub fn pointer_over(&self, cfg: Clay_ElementId) -> bool {
        unsafe { Clay_PointerOver(cfg) }
    }

    fn element_data(id: Clay_ElementId) -> Clay_ElementData {
        unsafe { Clay_GetElementData(id) }
    }

    pub fn bounding_box(&self, id: Clay_ElementId) -> Option<BoundingBox> {
        let element_data = Self::element_data(id);

        if element_data.found {
            Some(element_data.boundingBox.into())
        } else {
            None
        }
    }
}

impl<ImageElementData: Debug, CustomElementData: Debug, CustomLayoutSettings> Drop for LayoutEngine<ImageElementData, CustomElementData, CustomLayoutSettings> {
    fn drop(&mut self) {
        unsafe {
            Clay_SetCurrentContext(core::ptr::null_mut() as _);
        }
    }
}
