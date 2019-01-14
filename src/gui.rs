use azul::prelude::*;
use azul::widgets::{text_input::{TextInput, TextInputState}, button::Button};

const TEST_IMAGE: &[u8] = include_bytes!("../output.jpeg");
const CUSTOM_CSS: &str = "
    #cat { width: 500px; }
";

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
        let button = Button::with_label("Generate image").dom()
            .with_callback(On::LeftMouseUp, Callback(generate_image_callback));

        Dom::div().with_id("wrapper")
            .with_child(text_input)
            .with_child(image)
            .with_child(button)
    }
}

fn generate_image_callback(_app_state: &mut AppState<TestCrudApp>, window_info: WindowEvent<TestCrudApp>)
-> UpdateScreen
{
    println!("Button clicked!");
    UpdateScreen::Redraw
}

pub fn start() {
    let mut app = App::new(TestCrudApp::default(), AppConfig::default());
    app.add_image("Cat01", &mut TEST_IMAGE, ImageType::Jpeg).unwrap();
    let css = css::override_native(CUSTOM_CSS).unwrap();
    app.run(Window::new(WindowCreateOptions::default(), css).unwrap()).unwrap();
}