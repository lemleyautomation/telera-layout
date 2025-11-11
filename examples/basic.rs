#[allow(dead_code)]
#[derive(Debug,Default)]
enum Shapes {
    Line{width:f32},
    #[default]
    Circle
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct LayoutRenderer{
    pub mt: telera_layout::Vec2,
    s: String
}

impl telera_layout::MeasureText for LayoutRenderer {
    fn measure_text(&mut self, _text: &str, _text_config: telera_layout::TextConfig) -> telera_layout::Vec2 {
        telera_layout::Vec2 { x: 20.0, y: 12.0 }    
    }
}

impl LayoutRenderer {
    pub fn new() -> Self {
        Self { mt: telera_layout::Vec2 { x: 30.0, y: 30.0 }, s: "what's up".to_string() }
    }
}

fn main(){
    let mut layout_renderer = LayoutRenderer::new();

    layout_renderer.mt = telera_layout::Vec2 {x:20.0, y:12.0};

    let mut layout = telera_layout::LayoutEngine::<LayoutRenderer, (),(),()>::new((500.0,500.0));
    
    layout.begin_layout(layout_renderer);

    layout.open_element();

    let config = telera_layout::ElementConfiguration::new()
        .id("hi")
        .x_grow()
        .y_grow()
        .padding_all(5)
        .color(telera_layout::Color{r:5.0,g:7.0,b:9.0,a:255.0})
        .end();
    layout.configure_element(&config);

    let text_config = telera_layout::TextConfig::new()
        .font_id(0)
        .color(telera_layout::Color::default())
        .font_size(12)
        .line_height(14)
        .end();
    layout.add_text_element("hi1", &text_config, true);

    let text_config = telera_layout::TextConfig::new()
        .font_id(0)
        .color(telera_layout::Color::default())
        .font_size(12)
        .line_height(14)
        .end();
    layout.add_text_element("hi2", &text_config, true);

    let text_config = telera_layout::TextConfig::new()
        .font_id(0)
        .color(telera_layout::Color::default())
        .font_size(12)
        .line_height(14)
        .end();
    layout.add_text_element("hi3", &text_config, true);

    layout.open_element();
    let config = telera_layout::ElementConfiguration::new()
        .id("test")
        .x_fixed(50.0)
        .y_fixed(50.0)
        .color(telera_layout::Color::default())
        .end();
    layout.configure_element(&config);
    layout.close_element();

    layout.open_element();
    let config = telera_layout::ElementConfiguration::new()
        .x_grow()
        .y_grow()
        .color(telera_layout::Color::default())
        .end();
    layout.configure_element(&config);
    layout.close_element();

    layout.close_element();

    let (_render_commands, layout_renderer) = layout.end_layout();

    // Define an acceptable tolerance
    let epsilon: f32 = f32::EPSILON * 10.0;

    // Assert that the difference is within tolerance
    assert!(
        (layout_renderer.mt.x - 20.0).abs() < epsilon,
        "Values are not approximately equal: a = {}, b = {}",
        layout_renderer.mt.x,
        20.0
    );
    assert!(
        (layout_renderer.mt.y - 12.0).abs() < epsilon,
        "Values are not approximately equal: a = {}, b = {}",
        layout_renderer.mt.y,
        12.0
    );

    // for command in render_commands {
    //     match command {
    //         telera_layout::RenderCommand::Rectangle(rectangle) => println!("rect {:?}", rectangle.bounding_box),
    //         telera_layout::RenderCommand::Text(text) => println!("text {:?}", text.bounding_box),
    //         telera_layout::RenderCommand::Custom(custom) => println!("custom {:?}", custom.data),
    //         _ => ()
    //     }
    // }
}