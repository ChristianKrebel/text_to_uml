extern crate azul;

use self::azul::prelude::*;
use self::azul::widgets::text_input::*;

const TEST_IMAGE: &[u8] = include_bytes!("../output.jpeg");

struct TestCrudApp {
    text_input: TextInputState,
}

impl Default for TestCrudApp {
    fn default() -> Self {
        Self {
            text_input: TextInputState::new("Hover mouse over rectangle and press keys")
        }
    }
}

impl Layout for TestCrudApp {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div).with_id("wrapper")
            .with_child(TextInput::new()
                .bind(info.window, &self.text_input, &self)
                .dom(&self.text_input))
            .with_child(Dom::new(NodeType::Image(info.resources.get_image("Cat01").unwrap())).with_id("cat"))
    }
}

pub fn start() {
    let mut app = App::new(TestCrudApp::default(), AppConfig::default());
    app.add_image("Cat01", &mut TEST_IMAGE, ImageType::Jpeg).unwrap();
    app.run(Window::new(WindowCreateOptions::default(), css::native()).unwrap()).unwrap();
}