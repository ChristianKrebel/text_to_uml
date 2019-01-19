extern crate rusttype;
extern crate azul;
extern crate image;
extern crate imageproc;
extern crate rand;

pub(crate) mod parser;
pub(crate) mod generator;
mod gui;
mod defines;
use defines::*;

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