#[macro_use]
extern crate rusttype;

mod parser;
// load module file parser.rs
mod gui;
mod gui2;
mod generator;
mod defines;
use defines::*;
use std::string::*;
use std::env;

fn main() {
    gui::start();

    // --- Testing start ---
    /*let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();
    let mut content_lines: Vec<String> = Vec::new();
    let mut content_decor: Vec<TextDecoration> = Vec::new();
    content_lines.push(String::from("- Attribute"));
    content_decor.push(TextDecoration::None);
    let mut name: String = String::from("Klasse");*/

    /*let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();

    let args: Vec<String> = env::args().collect();

    let mut filename = "";
    let mut output_filename = "";

    if args.len() == 1 {
        filename = "input.txt";
        output_filename = "output.png";
    }else if args.len() == 2 {
        filename = args.get(1).unwrap();
        output_filename = "output.png";
    }else if args.len() == 3 {
        filename = args.get(1).unwrap();
        output_filename = args.get(2).unwrap();
    }

    parser::init(filename, &mut classes, &mut relations);

    //let mut class: Class = Class {class_type: ClassType::SimpleClass, class_name: name, border_width: 0, content_lines, content_decor};
    //classes.push(class);
    generator::generate_pic(&mut classes, &mut relations);
    // --- Testing end   ---
    */
}
