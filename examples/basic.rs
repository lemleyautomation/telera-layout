use std::cell::RefCell;
use std::rc::Rc;

use telera_layout::LayoutEngine;
use telera_layout::ElementConfiguration;
use telera_layout::TextConfig;
use telera_layout::Color;
use telera_layout::Vec2;
use telera_layout::RenderCommand;

#[repr(C)]
#[derive(Debug, Default)]
struct LayoutRenderer{
    pub mt: Vec2
}

fn measure_text(_text: &str, _text_config: &TextConfig, _user_data: &mut Rc<RefCell<LayoutRenderer>>) -> Vec2 {
    Vec2 { x: 20.0, y: 12.0 }
}

fn main(){
    let layout_renderer = Rc::<RefCell<LayoutRenderer>>::new(RefCell::new(LayoutRenderer::default()));

    layout_renderer.borrow_mut().mt = Vec2 {x:20.0, y:12.0};

    let mut layout = LayoutEngine::<(),(),()>::new((500.0,500.0));
    layout.set_text_measurement(layout_renderer.clone(), measure_text);

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