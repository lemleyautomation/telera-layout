use telera_layout::LayoutEngine;
use telera_layout::TextConfig;
use telera_layout::RenderCommand;

pub fn measure_text(_text: &str, _config: &TextConfig, _user_data: &mut f32) -> (f32, f32) {
    (20.0, 12.0)
}

fn main(){
    let mut layout = LayoutEngine::<(),()>::new((500.0,500.0));
    layout.set_text_measurement(3.0, measure_text);

    layout.begin_layout();

    let mut config = layout.open_element();

    config.id("hi");
    config.x_grow();
    config.y_grow();
    config.padding_all(5.0);
    config.background_color([100.0;4]);
    layout.config(config);

    let mut text_config = layout.open_text();
        text_config.font_id(0);
        text_config.color(0., 0., 0., 0.);
        text_config.font_size(12);
        text_config.line_height(14);
    layout.close_text("hi", text_config.end());

    layout.close_element();

    let render_commands = layout.end_layout();

    for command in render_commands {
        println!("{:?}", command);
    }

    // println!("hi");

    // for command in render_commands {
    //     println!("{:?}", command);
    //     match command.config {
    //         telera_layout::RenderCommandConfig::Rectangle(_) => {
    //             println!("wut");
    //         }
    //         telera_layout::RenderCommandConfig::None() => {
    //             println!("no")
    //         }
    //         _ => {}
    //     }
    // }
}