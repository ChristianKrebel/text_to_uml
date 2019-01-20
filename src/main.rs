extern crate rusttype;
extern crate azul;
extern crate image;
extern crate imageproc;
extern crate rand;

pub(crate) mod parser;
pub(crate) mod generator;
pub(crate) mod drawer;
mod reader;
mod gui;
mod defines;
use defines::*;

//========== Global constants ==========
pub const LINE_HEIGHT: u32 = 30;
pub const LETTER_WIDTH: u32 = 16;
pub const PADDING_LEFT: u32 = 8;
pub const PADDING_TOP: u32 = 2;
pub const RELATION_STICK: u32 = 50;
pub const DASHED_LENGTH: u32 = 5;
pub const DASHED_LENGTH2: u32 = DASHED_LENGTH * 5;
pub const REL_GAP_DISTANCE: f32 = 25.0;
pub const ARROW_SIZE: u32 = 20;
pub const ACTIVE_PADDING: u32 = PADDING_LEFT * 2;
pub const CARD_DIST: u32 = 4;
//========================================

fn main() {
    use std::path::Path;

    gui::start();

    // So sollten Bilder eigentlich geladen werden:
    // let (input_filename, output_filename) = get_cli_args("input.txt", "output.png");
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