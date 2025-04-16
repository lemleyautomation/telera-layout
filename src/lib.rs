mod bindings;
use bindings::clay::*;

mod type_substitutions;
use type_substitutions::*;

mod text_element;
use text_element::*;
pub use text_element::TextConfig;

mod config;
use config::*;

mod render_commands;
pub use render_commands::*;

use std::{
    fmt::Debug, marker::PhantomData, os::raw::c_void
};

unsafe extern "C" fn error_handler(error_data: Clay_ErrorData) {
    panic!("Clay Error: (type: {:?}) {:?}", error_data.errorType, error_data.errorText);
}


pub struct LayoutEngine<ImageElementData: Debug, CustomElementData: Debug>{
    _memory: Vec<u8>,
    context: *mut Clay_Context,
    text_measure_callback: Option<*const core::ffi::c_void>,
    _phantom: PhantomData<(CustomElementData, ImageElementData)>,
}


impl<ImageElementData: Debug + Default, CustomElementData: Debug + Default> LayoutEngine<ImageElementData, CustomElementData> {
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
            _phantom: PhantomData{}
        }
    }

    pub fn set_text_measurement<'clay, F, T>(
        &'clay mut self,
        userdata: T,
        callback: F,
    ) where
        F: Fn(&str, &TextConfig, &'clay mut T) -> (f32, f32) + 'static,
        T: 'clay,
    {
        // Box the callback and userdata together
        let boxed = Box::new((callback, userdata));

        // Get a raw pointer to the boxed data
        let user_data_ptr = Box::into_raw(boxed) as _;

        // Register the callback with the external C function
        unsafe {
            Self::set_measure_text_function_unsafe(
                measure_text_trampoline_user_data::<F, T>,
                user_data_ptr,
            );
        }

        // Store the raw pointer for later cleanup
        self.text_measure_callback = Some(user_data_ptr as *const core::ffi::c_void);
    }

    unsafe fn set_measure_text_function_unsafe(
        callback: unsafe extern "C" fn(
            Clay_StringSlice,
            *mut Clay_TextElementConfig,
            *mut core::ffi::c_void,
        ) -> Clay_Dimensions,
        user_data: *mut core::ffi::c_void,
    ) {
        Clay_SetMeasureTextFunction(Some(callback), user_data);
    }

    pub fn set_debug_mode(&self, enable: bool) {
        unsafe {
            Clay_SetDebugModeEnabled(enable);
        }
    }

    pub fn set_layout_dimensions(&self, dimensions: (f32,f32)) {
        unsafe {
            Clay_SetLayoutDimensions(dimensions.into());
        }
    }

    pub fn begin_layout(&self){
        unsafe { 
            Clay_BeginLayout();
            Clay_SetCurrentContext(self.context);
        };
    }

    pub fn end_layout<'commands>(&mut self) -> Vec<RenderCommand::<'commands, ImageElementData, CustomElementData>> {
        let array = unsafe {Clay_EndLayout()};
        
        let array = unsafe { core::slice::from_raw_parts(array.internalArray, array.length as usize) };

        let commands = array.iter().map(|command| {
            RenderCommand { 
                id: command.id, 
                bounding_box: [
                    command.boundingBox.x, 
                    command.boundingBox.y, 
                    command.boundingBox.width, 
                    command.boundingBox.height
                ], 
                z_index: command.zIndex,
                config: RenderCommandConfig::from(command),
            }
        }).collect::<Vec<RenderCommand::<ImageElementData, CustomElementData>>>();

        Vec::new()
    }

    pub fn open_element<'render_pass>(&self) -> ConfigBuilder<'render_pass, ImageElementData, CustomElementData>{
        unsafe {
            Clay__OpenElement();
        }
        ConfigBuilder::<'render_pass, ImageElementData, CustomElementData>::default()
    }

    pub fn config<'render_pass>(&self, config: ConfigBuilder<'render_pass, ImageElementData, CustomElementData>){
        unsafe {
            Clay__ConfigureOpenElement(config.into());
        }
    }
    
    pub fn close_element(&self){
        unsafe {
            Clay__CloseElement();
        }
    }

    pub fn open_text(&self) -> TextConfig{
        TextConfig::new()
    }

    pub fn close_text<'render>(&mut self, content: &'render str, config: TextElementConfig){
        unsafe { Clay__OpenTextElement(content.into(), config.into() ) };
    }

    

    pub fn pointer_state(&self, position: (f32,f32), is_down: bool) {
        unsafe {
            Clay_SetPointerState(position.into(), is_down);
        }
    }

    pub fn update_scroll_containers(
        &self,
        drag_scrolling_enabled: bool,
        scroll_delta: (f32, f32),
        delta_time: f32,
    ) {
        unsafe {
            Clay_UpdateScrollContainers(drag_scrolling_enabled, scroll_delta.into(), delta_time);
        }
    }

    pub fn scroll_container_data(&self, id: Id) -> Option<Clay_ScrollContainerData> {
        unsafe {
            Clay_SetCurrentContext(self.context);
            let scroll_container_data = Clay_GetScrollContainerData(id.into());

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

    pub fn pointer_over(&self, cfg: Id) -> bool {
        unsafe { Clay_PointerOver(cfg.into()) }
    }

    fn element_data(id: Id) -> Clay_ElementData {
        unsafe { Clay_GetElementData(id.into()) }
    }

    pub fn bounding_box(&self, id: Id) -> Option<(f32,f32,f32,f32)> {
        let element_data = Self::element_data(id);

        if element_data.found {
            Some(element_data.boundingBox.into())
        } else {
            None
        }
    }
}