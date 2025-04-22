use telera_layout::LayoutEngine;
use telera_layout::ElementConfiguration;
use telera_layout::TextConfig;
use telera_layout::MeasureText;
use telera_layout::Color;
use telera_layout::Vec2;
use telera_layout::RenderCommand;

#[repr(C)]
#[derive(Debug, Default)]
struct LayoutRenderer{
    pub mt: Vec2
}

impl MeasureText for LayoutRenderer {
    fn measure_text(&mut self, _text: &str, _text_config: TextConfig) -> Vec2 {
        Vec2 { x: 20.0, y: 12.0 }
    }
}

fn main(){
    let mut layout_renderer = LayoutRenderer::default();

    layout_renderer.mt = Vec2 {x:20.0, y:12.0};

    let mut layout = LayoutEngine::<(),(),()>::new((500.0,500.0));
    layout.set_text_measurement(&mut layout_renderer);

    layout.begin_layout();

    layout.open_element();

    let config = ElementConfiguration::new()
        .id("hi")
        .x_grow()
        .y_grow()
        .padding_all(5)
        .color(Color::default())
        .end();
    layout.configure_element(&config);

    let text_config = TextConfig::new()
        .font_id(0)
        .color(Color::default())
        .font_size(12)
        .line_height(14)
        .end();
    layout.add_text_element("hi", &text_config, true);

    layout.open_element();
    let config = ElementConfiguration::new()
        .id("test")
        .x_fixed(50.0)
        .y_fixed(50.0)
        .custom_layout_settings(&layout_renderer)
        .color(Color::default())
        .end();
    layout.configure_element(&config);
    layout.close_element();

    layout.close_element();

    let render_commands = layout.end_layout();

    for command in render_commands {
        match command {
            RenderCommand::Rectangle(rectangle) => println!("{:?}", rectangle.bounding_box),
            RenderCommand::Text(text) => println!("{:?}", text.bounding_box),
            RenderCommand::Custom(custom) => println!("{:?}", custom.custom_layout_settings),
            _ => ()
        }
    }
}