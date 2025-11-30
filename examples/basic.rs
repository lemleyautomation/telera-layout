use telera_layout::{Color, ElementConfiguration, LayoutEngine, MeasureText, RenderCommand, TextConfig, Vec2};

#[derive(Debug, Default)]
struct LayoutRenderer{
    pub mt: Vec2,
    s: String
}

impl MeasureText for LayoutRenderer {
    fn measure_text(&mut self, _text: &str, _text_config: TextConfig) -> Vec2 {
        self.mt.clone()
    }
}

impl LayoutRenderer {
    pub fn new() -> Self {
        Self { mt: Vec2 { x: 30.0, y: 30.0 }, s: "what's up".to_string() }
    }
}

fn main() {
    let mut layout_renderer = LayoutRenderer::new();

    let mut layout = LayoutEngine::<(),(),()>::new((500.0,500.0));
    
    layout.begin_layout();

    layout.open_element();

    let config = ElementConfiguration::new()
        .id("hi")
        .x_grow()
        .y_grow()
        .padding_all(5)
        .color(Color{r:5.0,g:7.0,b:9.0,a:255.0})
        .end();
    layout.configure_element(&config);

    let text_config = crate::TextConfig::new()
        .font_id(0)
        .color(crate::Color::default())
        .font_size(12)
        .line_height(14)
        .end();
    layout.add_text_element("hi1", &text_config, true, &mut layout_renderer);

    let text_config = crate::TextConfig::new()
        .font_id(0)
        .color(crate::Color::default())
        .font_size(45)
        .line_height(50)
        .end();
    layout.add_text_element("hi2", &text_config, true, &mut layout_renderer);

    let text_config = crate::TextConfig::new()
        .font_id(0)
        .color(crate::Color::default())
        .font_size(12)
        .line_height(14)
        .end();
    layout.add_text_element("hi3", &text_config, true, &mut layout_renderer);

    layout.open_element();
    let config = crate::ElementConfiguration::new()
        .id("test")
        .x_fixed(50.0)
        .y_fixed(50.0)
        .color(crate::Color::default())
        .end();
    layout.configure_element(&config);
    layout.close_element();

    layout.open_element();
    let config = crate::ElementConfiguration::new()
        .x_grow()
        .y_grow()
        .color(crate::Color::default())
        .end();
    layout.configure_element(&config);
    layout.close_element();

    layout.close_element();

    let render_commands = layout.end_layout(&mut layout_renderer);

    for command in render_commands {
        match command {
            RenderCommand::Rectangle(r) => println!("rectangle {:?}", r.bounding_box),
            RenderCommand::Text(t) => {
                println!("text {:?}", t.bounding_box);
            }
            _ => {}
        }
    }

    println!("{:?}", layout_renderer.s);

}