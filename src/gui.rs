extern crate azul;

use generator;
use parser;
use defines::*;
use gui2;

use self::azul::prelude::*;
use self::azul::{widgets::text_input::*, widgets::{label::Label, button::Button}};

const TEST_IMAGE: &[u8] = include_bytes!("../output.jpeg");

struct DataModel {
    text_input: TextInputState,
}

impl Default for DataModel {
    fn default() -> Self {
        Self {
            text_input: TextInputState::new("Please enter your model-text here"),
        }
    }
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        let btn_gen = Button::with_label("Generate model").dom()
            .with_callback(On::LeftMouseDown, Callback(generate_pic));
        Dom::new(NodeType::Div).with_id("wrapper")
            .with_child(TextInput::new()
                .bind(info.window, &self.text_input, &self)
                .dom(&self.text_input))
            .with_child(btn_gen)
            //.with_child(Dom::new(NodeType::Image(info.resources.get_image("Cat01").unwrap())).with_id("cat"))
    }
}

// View updates Model
fn generate_pic(app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> UpdateScreen {
    //app_state.data.modify(|state| state.counter += 1);
    let mut filename = "input.txt";
    let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();
    parser::init(filename, &mut classes, &mut relations);
    generator::generate_pic(&mut classes, &mut relations);

    /*let mut app2 = App::new(DataModel::default(), AppConfig::default());
    let mut window_options2 = WindowCreateOptions::default();
    window_options2.state.title = "Text to UML - Model".into();
    app2.push_window(Window::new(window_options2, css::native()).unwrap());
    //app2.create_window(WindowCreateOptions::default(), azul_native_style::native());
    */
    UpdateScreen::Redraw
}

pub fn start() {

    macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/hot_reload.css")) }

    let mut app = App::new(DataModel::default(), AppConfig::default());
    app.add_image("Cat01", &mut TEST_IMAGE, ImageType::Jpeg).unwrap();

    /*let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Text to UML".into();*/

    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let window = Window::new(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
    //app.run(Window::new(window_options, css::from_str(include_str!(CSS_PATH!())).unwrap())).unwrap();
}