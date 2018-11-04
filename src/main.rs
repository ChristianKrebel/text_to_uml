#[macro_use] extern crate conrod;

mod parser; // load module file parser.rs
mod gui;

fn main() {
    println!("{}", parser::example()); //use parser's function
    gui::start();
}
