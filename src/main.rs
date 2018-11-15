#[macro_use] extern crate conrod;
extern crate rusttype;
mod parser; // load module file parser.rs
mod gui;
mod generator;

fn main() {
    println!("{}", parser::example()); //use parser's function
    //gui::start();
    generator::test();
}
