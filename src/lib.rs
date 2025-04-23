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

mod element_configuration;
pub use element_configuration::ElementConfiguration;

// mod xml_parse;
// pub use xml_parse::*;

use std::{
    fmt::Debug, marker::PhantomData, os::raw::c_void
};

unsafe extern "C" fn error_handler(error_data: Clay_ErrorData) {
    panic!("Clay Error: (type: {:?}) {:?}", error_data.errorType, error_data.errorText);
}


pub struct LayoutEngine<ImageElementData: Debug, CustomElementData: Debug, CustomLayoutSettings>{
    _memory: Vec<u8>,
    context: *mut Clay_Context,
    text_measure_callback: Option<*const core::ffi::c_void>,
    _phantom: PhantomData<(CustomElementData, ImageElementData, CustomLayoutSettings)>,
    dangling_element_count: u32
}


impl<ImageElementData: Debug + Default, CustomElementData: Debug + Default, CustomLayoutSettings> LayoutEngine<ImageElementData, CustomElementData, CustomLayoutSettings> {
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
            text_measure_callback: None,
            _phantom: PhantomData{},
            dangling_element_count: 0
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

    fn check_for_dangling_elements(&self){
        if self.dangling_element_count != 0 || self.dangling_element_count%2 != 0  {
            panic!("all elements must have a configuration!")
        }
    }

    pub fn set_text_measurement<'clay, F, T>(
        &'clay mut self,
        userdata: T,
        callback: F,
    ) where
        F: Fn(&str, &TextConfig, &'clay mut T) -> Vec2 + 'static,
        T: 'clay,
    {
        // Box the callback and userdata together
        let boxed = Box::new((callback, userdata));

        // Get a raw pointer to the boxed data
        let user_data_ptr = Box::into_raw(boxed) as _;

        // Register the callback with the external C function
        unsafe {
            Clay_SetMeasureTextFunction(
                Some(measure_text_trampoline_user_data::<F,T>), 
                user_data_ptr
            );
            
        }

        // Store the raw pointer for later cleanup
        self.text_measure_callback = Some(user_data_ptr as *const core::ffi::c_void);
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

    pub fn begin_layout(&self){
        unsafe { 
            Clay_BeginLayout();
            Clay_SetCurrentContext(self.context);
        };
    }

    pub fn end_layout<'render_pass>(&mut self) -> Vec<RenderCommand::<'render_pass, ImageElementData, CustomElementData, CustomLayoutSettings>> {
        self.check_for_dangling_elements();

        let array = unsafe {Clay_EndLayout()};
        
        let array = unsafe { core::slice::from_raw_parts(array.internalArray, array.length as usize) };

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
        self.check_for_dangling_elements();

        unsafe {
            Clay__CloseElement();
        }
    }

    pub fn configure_element<'render_pass>(&mut self, config: &ElementConfiguration){
        self.undangle();
        unsafe {
            Clay__ConfigureOpenElement(config.into());
        }
    }
    
    pub fn add_text_element<'render_pass>(&mut self, content: &'render_pass str, config: &'render_pass TextConfig, statically_allicated: bool){
        self.check_for_dangling_elements();
        let text_config = unsafe { Clay__StoreTextElementConfig(config.into()) };
        unsafe { 
            Clay__OpenTextElement( 
                Clay_String { 
                    isStaticallyAllocated: statically_allicated, 
                    length: content.len() as i32, 
                    chars: content.as_ptr() as *const i8
                }, 
                text_config 
            ) 
        };
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

    unsafe extern "C" fn call_back_handler(id: Clay_ElementId, pointer_data: Clay_PointerData, user_data: isize){

    }

    pub fn on_click<callback: Fn()>(&mut self, callback_function: callback){
        unsafe {
            Clay_OnHover(Some(LayoutEngine::<(),(),()>::call_back_handler), 0);
        }
    }

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

/// macro to simplify layout creation
/// Causes code to be nested instead of flat
#[macro_export]
macro_rules! element {
    ( ($layout:expr), {$children:expr} ) => {
        
    };
}