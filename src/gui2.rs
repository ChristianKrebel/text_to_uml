extern crate azul;

use generator;
use parser;
use defines::*;

use self::azul::prelude::*;
use self::azul::{widgets::text_input::*, widgets::{label::Label, button::Button}};

const TEST_IMAGE: &[u8] = include_bytes!("../output.jpeg");

struct DataModel2 {
}

impl Default for DataModel2 {
    fn default() -> Self {
        Self {
        }
    }
}

impl Layout for DataModel2 {
    // Model renders View
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div).with_id("wrapper")
            .with_child(Dom::new(NodeType::Image(info.resources.get_image("Cat01").unwrap())).with_id("cat"))
    }
}

pub fn start() {
    let mut app2 = App::new(DataModel2::default(), AppConfig::default());
    app2.add_image("Cat01", &mut TEST_IMAGE, ImageType::Jpeg).unwrap();
    let mut window_options2 = WindowCreateOptions::default();
    window_options2.state.title = "Text to UML - Model".into();
    app2.run(Window::new(window_options2, css::native()).unwrap()).unwrap();
}