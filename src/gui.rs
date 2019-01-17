use azul::prelude::*;
use azul::prelude::RawImageFormat;
use azul::widgets::{text_input::{TextInput, TextInputState}, button::Button};
use parser;
use generator;

const CUSTOM_CSS: &str = "
    * { letter-spacing: 0.5pt; }
    #output_image { width: 500px; }
    #generate_button { height: 50px; }
    #placeholder_image { background-color: white; font-size: 20px; color: black; }
    #filename_wrapper { flex-direction: row; height: 28px; }
";
const IMAGE_ID: &str = "OutputImage";

#[derive(Default)]
struct TestCrudApp {
    input_file_name: TextInputState,
    output_file_name: TextInputState,
    current_image: Option<String>,
}

impl Layout for TestCrudApp {

    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {

        let input_file_name_text_field = TextInput::new()
            .bind(info.window, &self.input_file_name, &self)
            .dom(&self.input_file_name);

        let output_file_name_text_field = TextInput::new()
            .bind(info.window, &self.output_file_name, &self)
            .dom(&self.output_file_name);

        let file_names = Dom::div().with_id("filename_wrapper")
            .with_child(Dom::label("Input file name: "))
            .with_child(input_file_name_text_field)
            .with_child(Dom::label("Output file name: "))
            .with_child(output_file_name_text_field);

        let image = match &self.current_image {
            Some(image_id) => Dom::image(info.resources.get_image(image_id).unwrap()).with_id("output_image"),
            None => Dom::label("Please enter the file path and hit \"Generate Image\".").with_id("placeholder_image"),
        };

        let button = Button::with_label("Generate image").dom()
            .with_id("generate_button")
            .with_callback(On::LeftMouseUp, Callback(generate_image_callback));

        Dom::div().with_id("wrapper")
            .with_child(file_names)
            .with_child(image)
            .with_child(button)
    }
}

fn generate_image_callback(app_state: &mut AppState<TestCrudApp>, _window_info: WindowEvent<TestCrudApp>)
                           -> UpdateScreen
{
    use std::path::Path;
    use std::io::Cursor;
    use std::env;

    let old_image_id = app_state.data.lock().unwrap().current_image.clone();
    let current_input_path = app_state.data.lock().unwrap().input_file_name.text.clone();
    let current_output_path = app_state.data.lock().unwrap().output_file_name.text.clone();

    let current_working_directory = env::current_dir().ok().and_then(|p| Some(p.to_str().unwrap_or("/").to_string())).unwrap_or_default();
    let real_input_path = format!("{}/{}", current_working_directory, current_input_path);
    let real_output_path = format!("{}/{}", current_working_directory, current_output_path);

    // Delete the old image if necessary
    app_state.delete_image(IMAGE_ID);

    let (classes, relations) = match parser::init(&real_input_path) {
        Ok(cr) => cr,
        Err(e) => {
            println!("Error loading file: {}: {}", real_input_path, e);
            return UpdateScreen::DontRedraw;
        }
    };

    let (mut image_buf, dim) = generator::generate_pic(&classes, &relations);

    if real_output_path.is_empty()  {
        println!("Empty output file path!");
        return UpdateScreen::DontRedraw;
    }

    image_buf.save(&Path::new(&real_output_path))
        .unwrap_or_else(|_| { println!("Error saving file to: {}!", real_output_path); });

    let mut buffer = image_buf.into_raw();
    app_state.add_image_raw(IMAGE_ID, buffer, dim,RawImageFormat::BGRA8).unwrap();
    app_state.data.lock().unwrap().current_image = Some(IMAGE_ID.to_string());

    UpdateScreen::Redraw
}

pub fn start() {
    let app = App::new(TestCrudApp::default(), AppConfig::default());
    let css = css::override_native(CUSTOM_CSS).unwrap();
    app.run(Window::new(WindowCreateOptions::default(), css).unwrap()).unwrap();
}