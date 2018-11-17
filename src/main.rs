#[macro_use]
extern crate conrod;
extern crate rusttype;

mod parser;
// load module file parser.rs
mod gui;
mod generator;
mod defines;
use defines::*;
use std::string::*;

fn main() {
    //gui::start();

    // --- Testing start ---
    let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();
    let mut content_lines: Vec<String> = Vec::new();
    let mut content_decor: Vec<TextDecoration> = Vec::new();
    content_lines.push(String::from("- Attribute"));
    content_decor.push(TextDecoration::None);
    let mut name: String = String::from("Klasse");
    let mut class: Class = Class {class_type: ClassType::SimpleClass, class_name: name, border_width: 0, content_lines, content_decor};
    classes.push(class);
    generator::generate_pic(&mut classes, &mut relations);
    // --- Testing end   ---
}
