#[macro_use] extern crate conrod;

use conrod::backend::glium::glium::{self, Surface};

mod parser; // load module file parser.rs

fn main() {
    println!("{}", parser::example()); //use parser's function
}
