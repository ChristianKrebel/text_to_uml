use azul::prelude::*;
use azul::widgets::text_input::*;

const TEST_IMAGE: &[u8] = include_bytes!("../output.jpeg");

#[derive(Default)]
struct TestCrudApp {
    text_input: TextInputState,
}

impl Layout for TestCrudApp {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {

        let text_input = TextInput::new()
            .bind(info.window, &self.text_input, &self)
            .dom(&self.text_input);
        let image = Dom::image(info.resources.get_image("Cat01").unwrap()).with_id("cat");

        Dom::div()
        .with_id("wrapper")
            .with_child(text_input)
            .with_child(image)
    }
}

pub fn start() {
    let mut app = App::new(TestCrudApp::default(), AppConfig::default());
    app.add_image("Cat01", &mut TEST_IMAGE, ImageType::Jpeg).unwrap();
    app.run(Window::new(WindowCreateOptions::default(), css::native()).unwrap()).unwrap();
}