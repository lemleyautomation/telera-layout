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
    let mut layout = telera_layout::LayoutEngine::<LayoutRenderer, (),Shapes,()>::new((500.0,500.0));
    layout.begin_layout(layout_renderer);

    layout.open_element();
    let config = telera_layout::ElementConfiguration::new()
        .id("popup")
        .x_fixed(200.0)
        .y_fixed(200.0)
        .color(telera_layout::Color{r:0.0,g:255.0,b:0.0,a:255.0})
        .padding_all(25)
        .border_all(2)
        .radius_all(20.0)
        .floating()
        .floating_attach_element_at_center()
        .floating_attach_to_parent_at_center()
        .end();
    layout.configure_element(&config);
    layout.open_element();
    let config = telera_layout::ElementConfiguration::new()
        .color(telera_layout::Color{r:0.0,g:0.0,b:0.0,a:255.0})
        .x_grow()
        .y_grow()
        .custom_element(&Shapes::Line { width: 4.7 })
        .end();
    layout.configure_element(&config);
    layout.close_element();
    layout.close_element();

    let (render_commands, layout_renderer) = layout.end_layout();

    println!("{:?}", render_commands.len());

    for command in render_commands {
        match command {
            telera_layout::RenderCommand::Rectangle(rectangle) => println!("rect {:?}", rectangle.bounding_box),
            telera_layout::RenderCommand::Border(border) => println!("border {:?}", border.bounding_box),
            telera_layout::RenderCommand::Custom(custom) => println!("custom {:?}", custom.data),
            _ => ()
        }
    }
}