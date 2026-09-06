mod bindings;
use bindings::*;
pub use bindings::{
    Border, BorderWidth, BoundingBox, Color, CornerRadii, Custom, Image, Rectangle, RenderCommand,
    Vec2,
};
// clay types that appear in `LayoutEngine`'s public method signatures.
pub use bindings::{
    Clay_ElementId, Clay_PointerData, Clay_PointerDataInteractionState, Clay_ScrollContainerData,
    Clay_Vector2,
};

mod text_configuration;
pub use text_configuration::MeasureText;
pub use text_configuration::TextConfig;
use text_configuration::*;

mod element_configuration;
pub use element_configuration::ElementConfiguration;

use std::{fmt::Debug, marker::PhantomData, os::raw::c_void};

// Installed as clay's error handler in `LayoutEngine::build`; prints the error type
// and message to stdout.
unsafe extern "C" fn error_handler(error_data: Clay_ErrorData) {
    unsafe {
        let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
            error_data.errorText.chars as *const u8,
            error_data.errorText.length as _,
        ));

        println!("Clay Error: (type: {:?}) {:?}", error_data.errorType, text);
    }
}

/// Owns a clay context and its backing memory, and drives one layout at a time.
///
/// The type parameters fix the data threaded through render commands: `TextRenderer`
/// measures text (see [`MeasureText`]), `ImageElementData` / `CustomElementData` are
/// the referents passed to [`ElementConfiguration::image`] / `custom_element`, and
/// `CustomLayoutSettings` is the referent passed to
/// [`ElementConfiguration::custom_layout_settings`].
///
/// ```
/// use telera_layout::{Color, ElementConfiguration, LayoutEngine, MeasureText, TextConfig, Vec2};
///
/// struct NoText;
/// impl MeasureText for NoText {
///     fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 { Vec2::default() }
/// }
///
/// let mut engine = LayoutEngine::<NoText, (), (), ()>::new((800.0, 600.0));
///
/// let background = ElementConfiguration::new().grow_all().color(Color::rgb(0, 0, 0)).end();
///
/// engine.begin_layout(NoText);
/// engine.open_element();
/// engine.configure_element(&background);
/// engine.close_element();
/// let (commands, _renderer) = engine.end_layout();
/// assert!(!commands.is_empty());
/// ```
pub struct LayoutEngine<
    TextRenderer: MeasureText,
    ImageElementData: Debug,
    CustomElementData: Debug,
    CustomLayoutSettings,
> {
    _memory: Vec<u8>,
    context: *mut Clay_Context,
    /// The text renderer handed to [`LayoutEngine::begin_layout`], held for the
    /// duration of a layout pass so [`LayoutEngine::add_text_element`] can reach it,
    /// and handed back by [`LayoutEngine::end_layout`].
    text_renderer: Option<TextRenderer>,
    _phantom: PhantomData<(CustomElementData, ImageElementData, CustomLayoutSettings)>,
    dangling_element_count: u32,
}

impl<
    TextRenderer: MeasureText,
    ImageElementData: Debug,
    CustomElementData: Debug,
    CustomLayoutSettings,
> LayoutEngine<TextRenderer, ImageElementData, CustomElementData, CustomLayoutSettings>
{
    /// Creates an engine for a layout viewport of `dimensions` pixels `(width, height)`,
    /// allocating clay's arena from [`Clay_GetMaxElementCount`]-worth of memory. Use
    /// [`Self::with_max_element_count`] instead when the default capacity (8192
    /// elements) is not enough.
    ///
    /// ```
    /// use telera_layout::{LayoutEngine, MeasureText, TextConfig, Vec2};
    ///
    /// struct NoText;
    /// impl MeasureText for NoText {
    ///     fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 { Vec2::default() }
    /// }
    ///
    /// let engine = LayoutEngine::<NoText, (), (), ()>::new((1920.0, 1080.0));
    /// assert_eq!(engine.max_element_count(), 8192);
    /// ```
    ///
    /// [`Clay_GetMaxElementCount`]: Self::max_element_count
    pub fn new(dimensions: (f32, f32)) -> Self {
        Self::build(dimensions)
    }

    /// Like [`Self::new`] but overrides clay's element capacity before the backing
    /// arena is sized (clay's `Clay_SetMaxElementCount`, which must be set prior to
    /// `Clay_MinMemorySize` / `Clay_Initialize`). The measure-text word cache is
    /// sized by clay to `2 * max_element_count`.
    ///
    /// clay stores this as a process-global default, so construct the engine that
    /// needs a larger capacity before any other, and before its first layout.
    pub fn with_max_element_count(dimensions: (f32, f32), max_element_count: i32) -> Self {
        unsafe {
            // Detach any live context so clay writes the global default rather than
            // mutating another engine's context.
            Clay_SetCurrentContext(core::ptr::null_mut());
            Clay_SetMaxElementCount(max_element_count);
        }
        Self::build(dimensions)
    }

    // Allocates the arena, initializes a clay context and wraps it. Shared by `new`
    // and `with_max_element_count`.
    fn build(dimensions: (f32, f32)) -> Self {
        let memory_size = unsafe { Clay_MinMemorySize() as usize };
        let memory = vec![0; memory_size];
        let context;

        unsafe {
            let arena =
                Clay_CreateArenaWithCapacityAndMemory(memory_size, memory.as_ptr() as *mut c_void);

            context = Clay_Initialize(
                arena,
                Clay_Dimensions {
                    width: dimensions.0,
                    height: dimensions.1,
                },
                Clay_ErrorHandler {
                    errorHandlerFunction: Some(error_handler),
                    userData: std::ptr::null_mut(),
                },
            );
        }

        Self {
            _memory: memory,
            context,
            text_renderer: None,
            _phantom: PhantomData {},
            dangling_element_count: 0,
        }
    }

    // `open_element` increments this counter and `configure_element` decrements it, so
    // a non-zero value when closing an element or ending the layout means an element
    // was opened without being configured.
    fn dangle(&mut self) {
        self.dangling_element_count += 1;
    }

    fn undangle(&mut self) {
        if let Some(dangling_element_count) = self.dangling_element_count.checked_sub(1) {
            self.dangling_element_count = dangling_element_count;
        }
    }

    /// Updates the size of the layout viewport, e.g. after a window resize. Takes
    /// effect on the next [`Self::begin_layout`].
    pub fn set_layout_dimensions(&self, width: f32, height: f32) {
        unsafe {
            Clay_SetLayoutDimensions(Clay_Dimensions { width, height });
        }
    }

    /// Starts a layout pass: makes this engine's context current, registers
    /// `text_renderer` as clay's text-measurement function, and calls
    /// `Clay_BeginLayout`.
    ///
    /// The renderer is moved in and held until [`Self::end_layout`] returns it. While
    /// the pass is open, build the tree with [`Self::open_element`],
    /// [`Self::configure_element`], [`Self::add_text_element`] and
    /// [`Self::close_element`].
    ///
    /// ```
    /// use telera_layout::{ElementConfiguration, LayoutEngine, MeasureText, TextConfig, Vec2};
    ///
    /// struct Mono;
    /// impl MeasureText for Mono {
    ///     fn measure_text(&mut self, t: &str, c: TextConfig) -> Vec2 {
    ///         Vec2 { x: t.len() as f32 * c.font_size as f32 * 0.6, y: c.line_height as f32 }
    ///     }
    /// }
    ///
    /// let mut engine = LayoutEngine::<Mono, (), (), ()>::new((320.0, 240.0));
    /// let row = ElementConfiguration::new().grow_all().end();
    /// let label = TextConfig::new().font_size(16).line_height(20).end();
    ///
    /// engine.begin_layout(Mono);
    /// engine.open_element();
    /// engine.configure_element(&row);
    /// engine.add_text_element("hello", &label, true);
    /// engine.close_element();
    /// let (commands, _mono) = engine.end_layout();
    /// # assert!(!commands.is_empty());
    /// ```
    pub fn begin_layout(&mut self, text_renderer: TextRenderer) {
        self.text_renderer = Some(text_renderer);
        let ptr = self.text_renderer.as_mut().unwrap() as *mut TextRenderer as *mut c_void;
        unsafe {
            Clay_SetCurrentContext(self.context);
            Clay_SetMeasureTextFunction(Some(measure_text_c_callback::<TextRenderer>), ptr);
            Clay_BeginLayout();
        }
    }

    /// Finishes the layout pass, returning the render commands together with the
    /// text renderer that was handed to [`Self::begin_layout`].
    ///
    /// Panics if called without a matching [`Self::begin_layout`].
    pub fn end_layout<'render_pass>(
        &mut self,
    ) -> (
        Vec<RenderCommand<'render_pass, ImageElementData, CustomElementData, CustomLayoutSettings>>,
        TextRenderer,
    ) {
        assert!(
            self.text_renderer.is_some(),
            "end_layout called without a matching begin_layout"
        );
        assert!(
            self.dangling_element_count == 0 && self.dangling_element_count.is_multiple_of(2),
            "All elements must have a Configuration!"
        );

        // Re-point clay at the renderer in case the engine was moved since begin_layout;
        // Clay_EndLayout may still measure text during final layout.
        let ptr = self.text_renderer.as_mut().unwrap() as *mut TextRenderer as *mut c_void;
        let array = unsafe {
            Clay_SetMeasureTextFunction(Some(measure_text_c_callback::<TextRenderer>), ptr);
            let render_commands = Clay_EndLayout();
            Clay_SetMeasureTextFunction(None, std::ptr::null::<c_void>() as _);
            core::slice::from_raw_parts(
                render_commands.internalArray,
                render_commands.length as usize,
            )
        };

        let commands = array.iter().map(|command| {
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
        }).collect::<Vec<RenderCommand::<ImageElementData, CustomElementData, CustomLayoutSettings>>>();

        let text_renderer = self.text_renderer.take().unwrap();

        (commands, text_renderer)
    }

    /// Opens a new child of the currently open element (or a new root). Every
    /// `open_element` must be paired with a [`Self::configure_element`] and a
    /// [`Self::close_element`].
    pub fn open_element(&mut self) {
        self.dangle();
        unsafe {
            Clay__OpenElement();
        }
    }

    /// Closes the element opened by the most recent [`Self::open_element`], returning
    /// layout control to its parent. Panics if an element was opened but never
    /// configured.
    pub fn close_element(&mut self) {
        assert!(
            self.dangling_element_count == 0 && self.dangling_element_count.is_multiple_of(2),
            "All elements must have a Configuration!"
        );

        unsafe {
            Clay__CloseElement();
        }
    }

    /// Applies `config` to the currently open element and returns that element's id
    /// (the hash set with [`ElementConfiguration::id`], or a generated id if none was
    /// set).
    ///
    /// ```
    /// # use telera_layout::{ElementConfiguration, LayoutEngine, MeasureText, TextConfig, Vec2};
    /// # struct N; impl MeasureText for N { fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 { Vec2::default() } }
    /// let mut engine = LayoutEngine::<N, (), (), ()>::new((100.0, 100.0));
    /// let panel = ElementConfiguration::new().id("panel").grow_all().end();
    ///
    /// engine.begin_layout(N);
    /// engine.open_element();
    /// let id = engine.configure_element(&panel);
    /// engine.close_element();
    /// let _ = engine.end_layout();
    ///
    /// assert_eq!(id, engine.get_element_id("panel").id);
    /// ```
    pub fn configure_element(&mut self, config: &ElementConfiguration) -> u32 {
        self.undangle();
        unsafe {
            Clay__ConfigureOpenElement(config.into());
            Clay_GetOpenElementId()
        }
    }

    /// Adds a text element containing `content` to the currently open element, styled
    /// by `config`. `statically_allicated` tells clay whether `content` lives for the
    /// program's lifetime (a string literal) so it can skip copying it; when `false`,
    /// `content` must still outlive the [`Self::end_layout`] of this pass.
    ///
    /// Uses the text renderer handed to [`Self::begin_layout`]; panics if no layout
    /// pass is in progress.
    pub fn add_text_element(
        &mut self,
        content: &str,
        config: &TextConfig,
        statically_allicated: bool,
    ) {
        assert!(
            self.text_renderer.is_some(),
            "add_text_element called outside of a begin_layout / end_layout pass"
        );
        assert!(
            self.dangling_element_count == 0 && self.dangling_element_count.is_multiple_of(2),
            "All elements must have a Configuration!"
        );

        // Re-point clay at the renderer in case the engine was moved since begin_layout.
        let ptr = self.text_renderer.as_mut().unwrap() as *mut TextRenderer as *mut c_void;
        unsafe {
            Clay_SetMeasureTextFunction(Some(measure_text_c_callback::<TextRenderer>), ptr);
        }

        let text_config = unsafe { Clay__StoreTextElementConfig(config.into()) };
        unsafe {
            Clay__OpenTextElement(
                Clay_String {
                    isStaticallyAllocated: statically_allicated,
                    length: content.len() as i32,
                    chars: content.as_ptr() as *mut _,
                },
                text_config,
            )
        };
    }

    /// Records the pointer position `(x, y)` in layout space and whether its primary
    /// button is held. clay recomputes which elements are under the pointer and fires
    /// any [`Self::on_hover`] handlers from the previous layout. Call once per frame,
    /// before [`Self::update_scroll_containers`].
    pub fn pointer_state(&self, x: f32, y: f32, is_down: bool) {
        unsafe {
            Clay_SetPointerState(Clay_Vector2 { x, y }, is_down);
        }
    }

    /// Advances clay's built-in scrolling for clip containers under the pointer.
    /// `delta_x` / `delta_y` are this frame's scroll wheel / drag movement in pixels,
    /// `delta_time` is the frame duration in seconds (used for drag momentum), and
    /// `drag_scrolling_enabled` turns on touch-style drag-to-scroll.
    pub fn update_scroll_containers(
        &self,
        drag_scrolling_enabled: bool,
        delta_x: f32,
        delta_y: f32,
        delta_time: f32,
    ) {
        unsafe {
            Clay_UpdateScrollContainers(
                drag_scrolling_enabled,
                Clay_Vector2 {
                    x: delta_x,
                    y: delta_y,
                },
                delta_time,
            );
        }
    }

    /// The scroll offset clay applied to the currently open clip element this pass;
    /// pass it to [`ElementConfiguration::clip_child_offset`] on that element.
    pub fn get_scroll_offset(&self) -> Clay_Vector2 {
        unsafe { Clay_GetScrollOffset() }
    }

    /// Scroll position and content/container sizes for the clip element `id`, or
    /// `None` if no clip element with that id exists in the last layout.
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

    /// Whether the pointer set by [`Self::pointer_state`] is over the currently open
    /// element. Call between [`Self::configure_element`] and [`Self::close_element`].
    pub fn hovered(&self) -> bool {
        unsafe { Clay_Hovered() }
    }

    /// Registers a hover handler for the currently open element (clay's `Clay_OnHover`).
    /// Must be called while an element is open, i.e. between [`Self::configure_element`]
    /// and [`Self::close_element`].
    ///
    /// `callback` must be a plain function (no captures). `user_data` is threaded
    /// through unchanged: clay hands its address back to every invocation, so it must
    /// stay alive and pinned until the layout that registered the handler has been
    /// consumed by [`Self::end_layout`].
    ///
    /// clay invokes `callback` from within [`Self::pointer_state`] on the frame after
    /// the element was laid out, whenever the pointer is over it.
    ///
    /// ```
    /// # use telera_layout::{Clay_ElementId, Clay_PointerData, ElementConfiguration, LayoutEngine, MeasureText, TextConfig, Vec2};
    /// # struct N; impl MeasureText for N { fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 { Vec2::default() } }
    /// use std::sync::atomic::{AtomicBool, Ordering};
    ///
    /// extern "C" fn on_button(_id: Clay_ElementId, _p: Clay_PointerData, clicked: &AtomicBool) {
    ///     clicked.store(true, Ordering::SeqCst);
    /// }
    ///
    /// let clicked = AtomicBool::new(false);
    /// let mut engine = LayoutEngine::<N, (), (), ()>::new((100.0, 100.0));
    /// let button = ElementConfiguration::new().id("button").fixed(40.0, 20.0).end();
    ///
    /// engine.begin_layout(N);
    /// engine.open_element();
    /// engine.configure_element(&button);
    /// engine.on_hover(&clicked, on_button);
    /// engine.close_element();
    /// let _ = engine.end_layout();
    ///
    /// engine.pointer_state(10.0, 10.0, true);
    /// assert!(clicked.load(Ordering::SeqCst));
    /// ```
    pub fn on_hover<UserData>(
        &mut self,
        user_data: &UserData,
        callback: extern "C" fn(Clay_ElementId, Clay_PointerData, &UserData),
    ) {
        unsafe {
            let raw = core::mem::transmute::<
                extern "C" fn(Clay_ElementId, Clay_PointerData, &UserData),
                unsafe extern "C" fn(Clay_ElementId, Clay_PointerData, isize),
            >(callback);
            Clay_OnHover(Some(raw), user_data as *const UserData as isize);
        }
    }

    /// Whether the pointer set by [`Self::pointer_state`] is over the element with id
    /// `cfg`, based on the most recent layout. Unlike [`Self::hovered`] this works
    /// outside the build phase, given an id from [`Self::get_element_id`].
    pub fn pointer_over(&self, cfg: Clay_ElementId) -> bool {
        unsafe { Clay_PointerOver(cfg) }
    }

    /// Ids of every element currently under the pointer, in reverse z order
    /// (top-most first). Wraps clay's `Clay_GetPointerOverIds`.
    pub fn pointer_over_ids(&self) -> Vec<Clay_ElementId> {
        unsafe {
            let array = Clay_GetPointerOverIds();
            if array.internalArray.is_null() || array.length <= 0 {
                return Vec::new();
            }
            core::slice::from_raw_parts(array.internalArray, array.length as usize).to_vec()
        }
    }

    /// Hashes `id` into a [`Clay_ElementId`] the same way
    /// [`ElementConfiguration::id`] does, for use with [`Self::pointer_over`],
    /// [`Self::bounding_box`], [`Self::scroll_container_data`] and
    /// [`ElementConfiguration::floating_attach_to_element`].
    ///
    /// ```
    /// # use telera_layout::{LayoutEngine, MeasureText, TextConfig, Vec2};
    /// # struct N; impl MeasureText for N { fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 { Vec2::default() } }
    /// let engine = LayoutEngine::<N, (), (), ()>::new((10.0, 10.0));
    /// let a = engine.get_element_id("header");
    /// let b = engine.get_element_id("header");
    /// assert_eq!(a.id, b.id);
    /// ```
    pub fn get_element_id(&self, id: &str) -> Clay_ElementId {
        unsafe {
            Clay_GetElementId(Clay_String {
                isStaticallyAllocated: false,
                length: id.len() as i32,
                chars: id.as_ptr() as *const i8,
            })
        }
    }

    /// clay's `Clay_GetElementIdWithIndex` - like [`Self::get_element_id`] but folds
    /// `index` into the hash (matches the `id_indexed` element builder).
    pub fn get_element_id_with_index(&self, id: &str, index: u32) -> Clay_ElementId {
        unsafe {
            Clay_GetElementIdWithIndex(
                Clay_String {
                    isStaticallyAllocated: false,
                    length: id.len() as i32,
                    chars: id.as_ptr() as *const i8,
                },
                index,
            )
        }
    }

    // Raw `Clay_GetElementData`; returns a struct whose `found` flag is false when no
    // element matched.
    fn element_data(id: Clay_ElementId) -> Clay_ElementData {
        unsafe { Clay_GetElementData(id) }
    }

    /// `true` if an element with this id was present in the most recent layout.
    pub fn element_found(&self, id: Clay_ElementId) -> bool {
        Self::element_data(id).found
    }

    /// The final on-screen rectangle computed for element `id` in the most recent
    /// layout, or `None` if no element with that id was laid out.
    ///
    /// ```
    /// # use telera_layout::{ElementConfiguration, LayoutEngine, MeasureText, TextConfig, Vec2};
    /// # struct N; impl MeasureText for N { fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 { Vec2::default() } }
    /// let mut engine = LayoutEngine::<N, (), (), ()>::new((200.0, 200.0));
    /// let box_cfg = ElementConfiguration::new().id("box").fixed(50.0, 30.0).end();
    ///
    /// engine.begin_layout(N);
    /// engine.open_element();
    /// engine.configure_element(&box_cfg);
    /// engine.close_element();
    /// let _ = engine.end_layout();
    ///
    /// let bb = engine.bounding_box(engine.get_element_id("box")).unwrap();
    /// assert_eq!((bb.width, bb.height), (50.0, 30.0));
    /// assert!(engine.bounding_box(engine.get_element_id("missing")).is_none());
    /// ```
    pub fn bounding_box(&self, id: Clay_ElementId) -> Option<BoundingBox> {
        let element_data = Self::element_data(id);

        if element_data.found {
            Some(element_data.boundingBox.into())
        } else {
            None
        }
    }

    /// Enables / disables clay's debug overlay (clay's `Clay_SetDebugModeEnabled`).
    pub fn set_debug_mode_enabled(&self, enabled: bool) {
        unsafe { Clay_SetDebugModeEnabled(enabled) }
    }

    /// Whether the debug overlay is currently on (clay's `Clay_IsDebugModeEnabled`).
    pub fn is_debug_mode_enabled(&self) -> bool {
        unsafe { Clay_IsDebugModeEnabled() }
    }

    /// Enables / disables culling of render commands that fall outside the layout
    /// viewport (clay's `Clay_SetCullingEnabled`, on by default).
    pub fn set_culling_enabled(&self, enabled: bool) {
        unsafe { Clay_SetCullingEnabled(enabled) }
    }

    /// Hands scroll-offset management to the host application; pair with
    /// [`Self::set_query_scroll_offset_function`] (clay's experimental
    /// `Clay_SetExternalScrollHandlingEnabled`).
    pub fn set_external_scroll_handling_enabled(&self, enabled: bool) {
        unsafe { Clay_SetExternalScrollHandlingEnabled(enabled) }
    }

    /// Binds the callback clay uses to ask the host for the scroll offset of a clip
    /// element when external scroll handling is enabled (clay's experimental
    /// `Clay_SetQueryScrollOffsetFunction`). Same `user_data` lifetime rules as
    /// [`Self::on_hover`].
    pub fn set_query_scroll_offset_function<UserData>(
        &mut self,
        user_data: &UserData,
        callback: extern "C" fn(u32, &UserData) -> Clay_Vector2,
    ) {
        unsafe {
            let raw = core::mem::transmute::<
                extern "C" fn(u32, &UserData) -> Clay_Vector2,
                unsafe extern "C" fn(u32, *mut c_void) -> Clay_Vector2,
            >(callback);
            Clay_SetQueryScrollOffsetFunction(
                Some(raw),
                user_data as *const UserData as *mut c_void,
            );
        }
    }

    /// Max number of elements clay will allocate room for (clay's `Clay_GetMaxElementCount`).
    pub fn max_element_count(&self) -> i32 {
        unsafe { Clay_GetMaxElementCount() }
    }

    /// Max number of measured text words clay caches (clay's
    /// `Clay_GetMaxMeasureTextCacheWordCount`).
    pub fn max_measure_text_cache_word_count(&self) -> i32 {
        unsafe { Clay_GetMaxMeasureTextCacheWordCount() }
    }

    /// Clears the internal text-measurement cache, forcing every string to be
    /// re-measured on the next layout (clay's `Clay_ResetMeasureTextCache`). Useful
    /// after a font atlas / DPI change.
    pub fn reset_measure_text_cache(&self) {
        unsafe { Clay_ResetMeasureTextCache() }
    }
}

impl<
    TextRenderer: MeasureText,
    ImageElementData: Debug,
    CustomElementData: Debug,
    CustomLayoutSettings,
> Drop for LayoutEngine<TextRenderer, ImageElementData, CustomElementData, CustomLayoutSettings>
{
    /// Clears clay's current-context pointer so it cannot dangle into this engine's
    /// freed arena. clay keeps no per-context allocations of its own, so there is
    /// nothing else to release.
    fn drop(&mut self) {
        unsafe {
            Clay_SetCurrentContext(core::ptr::null_mut() as _);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use element_configuration::ElementConfiguration;
    use serial_test::serial;
    use std::sync::atomic::{AtomicU32, Ordering};

    struct FixedText;
    impl MeasureText for FixedText {
        fn measure_text(&mut self, _t: &str, _c: TextConfig) -> Vec2 {
            Vec2 { x: 10.0, y: 10.0 }
        }
    }

    type Engine = LayoutEngine<FixedText, (), (), ()>;

    #[test]
    #[serial]
    fn scalar_setters_and_queries_round_trip() {
        let engine = Engine::new((300.0, 300.0));

        assert_eq!(engine.max_element_count(), 8192);
        assert_eq!(engine.max_measure_text_cache_word_count(), 16384);

        engine.set_debug_mode_enabled(true);
        assert!(engine.is_debug_mode_enabled());
        engine.set_debug_mode_enabled(false);
        assert!(!engine.is_debug_mode_enabled());

        // just has to not panic / crash the C side
        engine.set_culling_enabled(false);
        engine.set_culling_enabled(true);
        engine.set_external_scroll_handling_enabled(false);
        engine.reset_measure_text_cache();

        let a = engine.get_element_id("cell");
        let b = engine.get_element_id_with_index("cell", 1);
        assert_ne!(a.id, b.id);
        assert_eq!(b.offset, 1);
    }

    #[test]
    #[serial]
    fn with_max_element_count_takes_effect() {
        let engine = Engine::with_max_element_count((100.0, 100.0), 256);
        assert_eq!(engine.max_element_count(), 256);
        // clay derives the text-word cache as 2x
        assert_eq!(engine.max_measure_text_cache_word_count(), 512);
        // restore the default for other tests
        drop(engine);
        let _ = Engine::with_max_element_count((1.0, 1.0), 8192);
    }

    extern "C" fn record_hover(id: Clay_ElementId, _p: Clay_PointerData, hit: &AtomicU32) {
        hit.store(id.id, Ordering::SeqCst);
    }

    /// A renderer that counts how many times clay asked it to measure text, and the
    /// concatenation of every string it saw.
    #[derive(Default)]
    struct RecordingText {
        calls: u32,
        seen: String,
    }
    impl MeasureText for RecordingText {
        fn measure_text(&mut self, text: &str, _c: TextConfig) -> Vec2 {
            self.calls += 1;
            self.seen.push_str(text);
            Vec2 { x: 7.0, y: 7.0 }
        }
    }

    #[test]
    #[serial]
    fn begin_layout_owns_and_end_layout_returns_the_text_renderer() {
        let mut engine =
            LayoutEngine::<RecordingText, (), (), ()>::with_max_element_count((100.0, 100.0), 8192);

        engine.begin_layout(RecordingText::default());
        engine.open_element();
        let cfg = ElementConfiguration::new().grow_all().end();
        engine.configure_element(&cfg);
        let text_cfg = TextConfig::new().end();
        engine.add_text_element("hello ", &text_cfg, true);
        engine.add_text_element("world", &text_cfg, true);
        engine.close_element();

        let (commands, renderer) = engine.end_layout();

        assert!(renderer.calls >= 2, "clay should have measured both strings");
        assert!(renderer.seen.contains("hello"));
        assert!(renderer.seen.contains("world"));
        assert!(
            commands
                .iter()
                .any(|c| matches!(c, RenderCommand::Text(_))),
            "expected a text render command"
        );

        // The engine is reusable: a second pass takes a fresh renderer.
        engine.begin_layout(RecordingText::default());
        engine.open_element();
        engine.configure_element(&cfg);
        engine.close_element();
        let (_c2, renderer2) = engine.end_layout();
        assert_eq!(renderer2.calls, 0);
    }

    #[test]
    #[serial]
    #[should_panic(expected = "begin_layout")]
    fn add_text_element_without_begin_layout_panics() {
        let mut engine = Engine::new((10.0, 10.0));
        let text_cfg = TextConfig::new().end();
        engine.add_text_element("nope", &text_cfg, true);
    }

    #[test]
    #[serial]
    fn on_hover_fires_and_pointer_over_ids_report_the_element() {
        let hit = AtomicU32::new(0);
        let mut engine = Engine::with_max_element_count((200.0, 200.0), 8192);

        let target = engine.get_element_id("target");

        engine.begin_layout(FixedText);
        engine.open_element();
        let cfg = ElementConfiguration::new()
            .id("target")
            .fixed(80.0, 80.0)
            .end();
        engine.configure_element(&cfg);
        engine.on_hover(&hit, record_hover);
        engine.close_element();
        let (_commands, _text) = engine.end_layout();

        // Pointer inside the 80x80 box anchored at the origin.
        engine.pointer_state(40.0, 40.0, false);

        assert_eq!(hit.load(Ordering::SeqCst), target.id);
        assert!(engine.pointer_over(target));
        assert!(engine.pointer_over_ids().iter().any(|e| e.id == target.id));
    }
}
