use azul::prelude::*;
use azul::prelude::RawImageFormat;
use azul::widgets::{button::Button, label::Label, text_input::{TextInput, TextInputState}};
use generator;
use parser;
use defines::*;

const CUSTOM_CSS: &str = "
    * { letter-spacing: 0.5pt; }
    #input_field { padding-left: 4px; padding-right: 4px; }
    #input_label { padding-left: 4px; padding-right: 4px; }
    #output_field { padding-left: 4px; padding-right: 4px; }
    #output_label { padding-left: 4px; padding-right: 4px; }
    #input_model_field { height: 400px; min-width: 200px; background-color: yellow; }
    #output_image {  }
    #generate_button {  }
    #status_label { background-color: red; line-height: 1.3pt; }
    #placeholder_image { background-color: blue; font-size: 20px; color: black; }
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
            input_model_structure: TextInputState::new("AbstractClass:Person
--
protected static String name
protected String vorname
--
public String getFullName()

Class:Angestellter
--
static int ID
private String position
--
public Auftrag auftragErstellen()
public void auftragBearbeiten()

"),
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
            None => Dom::label("Please enter the file path and hit \"Generate Image\".")
                .with_id("placeholder_image"),
        };

        let button = Button::with_label("Generate image").dom()
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

    // Delete the old image if necessary
    app_state.delete_image(IMAGE_ID);


    // Clear status
    app_state.data.modify(|state| state.status = String::from(""));

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
    let (classes, relations) = match parser::init
        (if !current_input_field.is_empty() { &current_input_field } else { &real_input_path },
         !current_input_field.is_empty()
        )
        {
        Ok(cr) => cr,
        Err(e) => {
            println!("ERROR: Cannot load file \"{}\": {}.", real_input_path, e);
            app_state.data.modify(|state| state.status =
                format!("{}ERROR: Cannot load file \"{}\": {}.\n", state.status, real_input_path, e));
            return UpdateScreen::Redraw;
        }
    };
    //========================================

    //========== Test Implementation ==========
    /*let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();
    let mut content_lines: Vec<String> = Vec::new();
    let mut content_decor: Vec<TextDecoration> = Vec::new();
    content_lines.push(String::from("-"));
    content_lines.push(String::from("- Attribute"));
    content_decor.push(TextDecoration::None);
    content_decor.push(TextDecoration::None);
    let mut content_lines2: Vec<String> = Vec::new();
    let mut content_decor2: Vec<TextDecoration> = Vec::new();
    content_lines2.push(String::from("-"));
    content_lines2.push(String::from("- Attribute"));
    content_decor2.push(TextDecoration::None);
    content_decor2.push(TextDecoration::Underlined);
    let mut class: Class = Class
        {
            class_type: ClassType::SimpleClass,
            class_name: String::from("Klasse"),
            class_stereotype: String::from("<<interface>>"),
            border_width: 0,
            content_lines: content_lines,
            content_decor: content_decor
        };
    let mut class2: Class = Class
        {
            class_type: ClassType::SimpleClass,
            class_name: String::from("Klasse2"),
            class_stereotype: String::from("<<blub>>"),
            border_width: 0,
            content_lines: content_lines2,
            content_decor: content_decor2
        };
    classes.push(class);
    classes.push(class2);

    let mut relation: Relation = Relation
        {
            border_type: BorderType::Dashed,
            arrow_type: RelationArrow::DiamondFilled,
            from_class: String::from("Klasse"),
            from_class_card: String::from("n"),
            to_class: String::from("Klasse2"),
            to_class_card: String::from("*")
        };
    relations.push(relation);*/
    //========================================

    let (mut image_buf, dim) = generator::generate_pic(
        &classes, &relations,
    );

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

    let mut buffer = image_buf.into_raw();
    app_state.add_image_raw(IMAGE_ID, buffer, dim,
                            RawImageFormat::BGRA8).unwrap();
    app_state.data.lock().unwrap().current_image = Some(IMAGE_ID.to_string());

    app_state.data.modify(|state| state.status =
        format!("{}SUCCESS: Generated model.\n", state.status));

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