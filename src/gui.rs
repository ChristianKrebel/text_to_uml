use azul::prelude::*;
use azul::prelude::RawImageFormat;
use azul::widgets::{button::Button, label::Label, text_input::{TextInput, TextInputState}};
use generator;
use parser;
use drawer;
use reader;
use defines::*;
use std::str::*;

const CUSTOM_CSS: &str = "
    * { letter-spacing: 0.5pt; }
    #input_field { padding-left: 4px; padding-right: 4px; }
    #input_label { padding-left: 4px; padding-right: 4px; }
    #output_field { padding-left: 4px; padding-right: 4px; }
    #output_label { padding-left: 4px; padding-right: 4px; }
    #input_model_field { line-height: 1.3pt; min-height: 600px; min-width: 200px; }
    #output_image {  }
    #generate_button { max-width: 300px; }
    #status_label { line-height: 1.3pt; height: 70px; text-align: left; margin-left: 10px; margin-right: 10px; }
    #placeholder_image { line-height: 1.3pt; margin-left: 10px; margin-right: 10px; font-size: 20px; color: black; }
    #filename_wrapper { flex-direction: row; height: 28px; padding: 4px; margin: 2px; }
    #bottom_wrapper { min-height: 200px; flex-direction: row; padding: 4px; margin: 2px; }
    #middle_wrapper { flex-direction: row; min-height: 50px; max-height: 70px; padding: 4px; margin: 2px; }
";
const IMAGE_ID: &str = "OutputImage";

struct AppData {
    input_file_name: TextInputState,
    output_file_name: TextInputState,
    input_model_structure: TextInputState,
    current_image: Option<String>,
    status: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            input_file_name: TextInputState::new("input.txt"),
            output_file_name: TextInputState::new("output.png"),
            input_model_structure: TextInputState::new("Model:Object

Object:lg
lieblingsgrieche:Restaurant
kategorie:Sterne 3
name \"Platon\"

Object:maren
maren:Gast
status \"König\"
geldbetrag:EUR 300

Object:klaudia
klaudia:Gast
status \"König\"
geldbetrag:EUR 20
hunger true

Object:k1
:Kellner
persAusweisNr 12345
gehalt:EUR 1500

Link
k1,lg
+Arbeitnehmer,+Arbeitgeber

Link:bedient
k1,maren

Link:bedient
k1,klaudia

Link:besucht
klaudia,lg

Link:besucht
maren,lg

/Model"),
            current_image: None,
            status: String::new()
        }
    }
}

impl Layout for AppData {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        let input_file_name_text_field = TextInput::new()
            .bind(info.window, &self.input_file_name, &self)
            .dom(&self.input_file_name);

        let output_file_name_text_field = TextInput::new()
            .bind(info.window, &self.output_file_name, &self)
            .dom(&self.output_file_name);

        let file_names = Dom::div().with_id("filename_wrapper")
            .with_child(Dom::label("Input file name: ").with_id("input_label"))
            .with_child(input_file_name_text_field.with_id("input_field"))
            .with_child(Dom::label("Output file name: ").with_id("output_label"))
            .with_child(output_file_name_text_field.with_id("output_field"));

        let input_model_structure_text_field = TextInput::new()
            .bind(info.window, &self.input_model_structure, &self)
            .dom(&self.input_model_structure)
            .with_id("input_model_field");

        let image = match &self.current_image {
            Some(image_id) => Dom::image(info.resources.get_image(image_id).unwrap())
                .with_id("output_image"),
            None => Dom::label("Please enter the file path(s) or the text directly into the text field and hit \"Generate Image\".")
                .with_id("placeholder_image"),
        };

        let button = Button::with_label("Generate Image").dom()
            .with_id("generate_button")
            .with_callback(On::LeftMouseDown, Callback(generate_image_callback));

        let status_label = Label::new(format!("{}", self.status)).dom()
            .with_id("status_label");

        Dom::div().with_id("wrapper")
            .with_child(file_names)
            .with_child(Dom::div().with_id("middle_wrapper")
                .with_child(button)
                .with_child(status_label)
            )
            .with_child(Dom::div().with_id("bottom_wrapper")
                .with_child(input_model_structure_text_field)
                .with_child(image)
            )
    }
}

fn generate_image_callback(app_state: &mut AppState<AppData>, _window_info: WindowEvent<AppData>)
                           -> UpdateScreen
{
    use std::path::Path;
    use std::io::Cursor;
    use std::env;
    let old_image_id = app_state.data.lock().unwrap().current_image.clone();
    let current_input_path = app_state.data.lock().unwrap().input_file_name.text.clone();
    let current_output_path = app_state.data.lock().unwrap().output_file_name.text.clone();
    let current_input_field= app_state.data.lock().unwrap().input_model_structure.text.clone();

    let current_working_directory = env::current_dir().ok()
        .and_then(|p| Some(p.to_str().unwrap_or("/").to_string())).unwrap_or_default();
    let real_input_path = format!("{}/{}", current_working_directory, current_input_path);
    let real_output_path = format!("{}/{}", current_working_directory, current_output_path);


    // Clear status
    app_state.data.modify(|state| state.status = String::from(""));

    // Error and return if there's no input
    if current_input_field.is_empty() && current_input_path.is_empty() {
        println!("ERROR: Cannot find input");
        app_state.data.modify(|state| state.status =
            format!("{}ERROR: Cannot find input\n", state.status));
        return UpdateScreen::Redraw;
    }



    // Check for file extension
    let mut dot_pos = 0;
    let mut file_extension: String = String::from("");

    if current_input_path.contains(".") {
        dot_pos = current_input_path.chars().position(|c| c == '.').unwrap() + 1;
        let file_extension_len = current_input_path.len() - dot_pos;
        file_extension = current_input_path.chars().skip(dot_pos).take(file_extension_len).collect();
    }
    if !current_input_path.is_empty() {
        if !current_input_path.contains(".")
            || file_extension.is_empty()
            || !file_extension.chars().all(|x| x.is_alphabetic())
            {
                println!("ERROR: Cannot load file \"{}\": No (correct) file extension found.",
                         real_input_path);
                app_state.data.modify(|state| state.status =
                    format!("{}ERROR: Cannot load file \"{}\": No (correct) file extension found.\n",
                            state.status, real_input_path));
                return UpdateScreen::Redraw;
            }
    }

    //========== Correct Implementation ==========

    // Get lines either from file or from text input

    let mut lines: Vec<String> = Vec::new();

    if !current_input_field.is_empty() {
        lines = match reader::read_from_text(&current_input_field) {
            Ok(val) => val,
            Err(err) => {
                app_state.data.modify(|state| state.status =
                    format!("{}ERROR: Cannot read input text: {}\n", state.status, err));
                return UpdateScreen::Redraw;
            }
        };
    } else{
        lines = match reader::read_from_file(&real_input_path) {
            Ok(val) => val,
            Err(err) => {
                app_state.data.modify(|state| state.status =
                    format!("{}ERROR: Cannot read from file \"{}\": {}\n", state.status, real_input_path, err));
                return UpdateScreen::Redraw;
            }
        };
    }


    for line in lines.iter(){
        println!("lines: {}", line);
    }

    // Get ModelContainer with one model and its type

    let model = match parser::parse_model(&lines) {
        Ok(val) => val,
        Err(err) => {
            app_state.data.modify(|state| state.status =
                format!("{}ERROR: Cannot not parse Input: {}\n", state.status, err));
            return UpdateScreen::Redraw;
        }
    };


    // Get a buffer with the drawn model and the picture dimensions

    let (mut image_buf, mut dim) = drawer::get_image(model);

    let (dima, dimb) = dim;
    println!("{}, {}", dima, dimb);


    //========================================


    // Save the image if possible

    if current_output_path.is_empty() {
        println!("ERROR: The output file path cannot be empty.");
        app_state.data.modify(|state| state.status =
            format!("{}ERROR: The output file path cannot be empty.\n", state.status));
    }

    image_buf.save(&Path::new(&real_output_path))
        .unwrap_or_else(|_|
            {
                println!("ERROR: Cannot save file to \"{}\".", real_output_path);
                app_state.data.modify(|state| state.status =
                    format!("{}ERROR: Cannot save file to \"{}\".\n", state.status, real_output_path));
            });

    // Update the shown picture in GUI
    // Delete the old image if necessary
    app_state.delete_image(IMAGE_ID);
    let mut buffer = image_buf.into_raw();
    app_state.add_image_raw(IMAGE_ID, buffer, dim,
                            RawImageFormat::BGRA8).unwrap();
    app_state.data.lock().unwrap().current_image = Some(IMAGE_ID.to_string());

    app_state.data.modify(|state| state.status =
        format!("{}SUCCESS: Generated model.\n", state.status));

    println!("Redraw");
    UpdateScreen::Redraw
}

pub fn start() {
    let app = App::new(AppData::default(), AppConfig::default());
    let css = css::override_native(CUSTOM_CSS).unwrap();



    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "TextToUML".into();
    window_options.state.is_maximized = true.into();
    app.run(Window::new(window_options, css).unwrap()).unwrap();
}
