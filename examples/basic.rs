use telera_layout::LayoutEngine;
use telera_layout::ElementConfiguration;
use telera_layout::TextConfig;
use telera_layout::Color;
use telera_layout::Vec2;
use telera_layout::RenderCommand;

#[derive(Debug, Default)]
struct Custom{
    a: u8
}

#[derive(Debug, Default)]
struct Layout{
    b: u8
}

pub fn measure_text(_text: &str, _config: &TextConfig, _user_data: &mut Option<()>) -> Vec2 {
    Vec2 { x: 20.0, y: 12.0 }
}

fn main(){
    let custom = Custom {a: 0};
    let custom_layout = Layout {b:0};

    let mut layout = LayoutEngine::<(),Custom,Layout>::new((500.0,500.0));
    layout.set_text_measurement(None, measure_text);

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
        .custom_layout_settings(&custom_layout)
        .color(Color::default())
        .custom_element(&custom)
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