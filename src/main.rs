extern crate rusttype;
extern crate azul;
extern crate image;
extern crate imageproc;
extern crate rand;
#[macro_use]
extern crate nom;

pub(crate) mod parser;
pub(crate) mod generator;
pub(crate) mod drawer;
mod reader;
mod gui;
mod defines;
use std::str::*;
use std::env::*;
use defines::*;

//========== Global constants ==========
pub const LINE_HEIGHT: u32 = 30;
pub const LETTER_WIDTH: u32 = 16;
pub const LETTER_WIDTH_ACCURATE: u32 = LETTER_WIDTH - 4;
pub const PADDING_LEFT: u32 = 8;
pub const PADDING_TOP: u32 = 2;
pub const RELATION_STICK: u32 = 50;
pub const DASHED_LENGTH: u32 = 5;
pub const DASHED_LENGTH2: u32 = DASHED_LENGTH * 5;
pub const REL_GAP_DISTANCE: f32 = 25.0;
pub const LINK_GAP_DISTANCE: f32 = 55.0;
pub const ARROW_SIZE: u32 = 20;
pub const ACTIVE_PADDING: u32 = PADDING_LEFT * 2;
pub const CARD_DIST: u32 = 4;
pub const ROLE_NAME_DIST: u32 = 8;
pub const ROLE_NAME_ARROW_SIZE: u32 = 15;
//========================================

fn main() {
    use std::path::Path;

    gui::start();
// Class:Person\n<<abstract>>\n--\nprotected String name\n--\npublic abstract void shoutName()\n\nClass:Chinese\n--\nprotected String name\n--\npublic void shoutName()\n\nInheritance\nChinese,Person\n1,1\n/Model

    /*
Model:Class

AbstractClass:Person
--
private String name
private String vorname
--
public static String getFullName()

Class:Angestellter
--
private static int ID
private String<ll> position
--
public Auftrag auftragErstellen()
public void auftragBearbeiten<lol>()

Class:Auftrag
--
private MyList<Item> inhalt
private boolean done
--
public void setDone(booleanisDone)
public int getCumulativePrice()
public ArrayList<Item> getInhalt()

Class:Item
--
private String description

/Model
*/



//    let lines = vec!["Model:Class".to_string(),
//                     "AbstractClass:Person".to_string(),
//                     "--".to_string(),
//                     "protected String name".to_string(),
//                     "protected String vorname".to_string(),
//                     "--".to_string(),
//                     "public static String getFullName()".to_string(),
//                     "".to_string(),
//                     "Class:Angestellter".to_string(),
//                     "--".to_string(),
//                     "private static int ID".to_string(),
//                     "private String<ll> position".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "".to_string(),
//                     "/Model".to_string()];
//    let current_working_directory = current_dir().ok().and_then(|p| Some(p.to_str().unwrap_or("/").to_string())).unwrap_or_default();;
//    let stringo = match reader::read_from_file(&*format!("{}/{}", current_working_directory, "input_class.txt")) {
//        Ok(val) => val,
//        Err(err) => return,
//    };
//    let mc = match parser::parse_model(&stringo) {
//        Ok(val) => val,
//        Err(err) => {
//            println!("Encountered error while parsing: {}", err);
//            return;
//        }
//    };
//
//    println!("Verifying class model information: {:?}", mc.class_model.unwrap());

//    match parser::parse_model(&lines) {
//        Ok(val) => println!("Parsing successful! {:?}\n", val),
//        Err(err) => println!("ERROR parsing the text input: {}\n", err)
//    };

    // So sollten Bilder eigentlich geladen werden:
    // let (input_filename, output_filename) = get_cli_args("input_class.txt_class", "output.png");
    // let (classes, relations) = parser::init(&input_filename).unwrap();
    // let image_buf = generator::generate_pic(&classes, &relations);
    // image_buf.save(&Path::new(&output_filename)).unwrap();

}

type InputFilePath = String;
type OutputFilePath = String;

fn get_cli_args(default_input_path: &str, default_output_path: &str)
                -> (InputFilePath, OutputFilePath)
{
    use std::env;

    let input_filename = env::args().nth(1).unwrap_or(default_input_path.to_string());
    let output_filename = env::args().nth(2).unwrap_or(default_output_path.to_string());
    (input_filename, output_filename)
}