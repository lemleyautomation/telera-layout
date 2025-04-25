use std::cell::RefCell;
use std::rc::Rc;
use std::fs::read_to_string;

use telera_layout::LayoutEngine;
use telera_layout::ElementConfiguration;
use telera_layout::ParserDataAccess;
use telera_layout::TextConfig;
use telera_layout::MeasureText;
use telera_layout::Color;
use telera_layout::Vec2;
use telera_layout::RenderCommand;
use telera_layout::Parser;
use telera_layout::EnumString;

#[repr(C)]
#[derive(Debug, Default)]
struct LayoutRenderer{
    pub mt: Vec2,
    s: String
}

impl MeasureText for LayoutRenderer {
    fn measure_text(&mut self, text: &str, text_config: TextConfig) -> Vec2 {
        //println!("{:?}", self.s);
        Vec2 { x: 20.0, y: 12.0 }    
    }
}

impl LayoutRenderer {
    pub fn new() -> Self {
        Self { mt: Vec2 { x: 30.0, y: 30.0 }, s: "what's up".to_string() }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, EnumString)]
enum Events{
    #[default]
    None
}

struct App{}

impl ParserDataAccess<(),Events> for App{}

fn main(){
    let mut layout_renderer = LayoutRenderer::new();
    layout_renderer.mt = Vec2 {x:20.0, y:12.0};

    let mut user_app = App{};

    let mut pages = Parser::<Events>::default();
    let file = "examples/layout.xml";
    let file = read_to_string(file).unwrap();
    pages.add_page(&file).unwrap();

    let mut layout = LayoutEngine::<LayoutRenderer, (),(),()>::new((500.0,500.0));

    layout.begin_layout(layout_renderer);

    pages.set_page("Main", false, &mut layout, &mut user_app);

    let (render_commands, mut layout_renderer) = layout.end_layout();

    layout_renderer.mt.x = 4.0;

    for command in render_commands {
        match command {
            RenderCommand::Rectangle(rectangle) => println!("{:?}", rectangle.bounding_box),
            RenderCommand::Text(text) => println!("{:?}", text.bounding_box),
            RenderCommand::Custom(custom) => println!("{:?}", custom.custom_layout_settings),
            _ => ()
        }
    }
}