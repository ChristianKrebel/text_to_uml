#![allow(dead_code)]

use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::io;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io::prelude::*;
use std::str::*;

use defines::*;


// Testing Modul. Delete this function and implement your own Hannes.
pub fn init() -> String{

    let mut vec: Vec<String> = Vec::new();
    read_file(&mut vec);

    let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();

    parse_lines(&mut vec, &mut classes, &mut relations);


    for line in vec.iter(){
        println!("lines: {}", line);
    }

    let str = String::from("done reading file.");
    return str;
}

fn parse_lines(lines: &mut Vec<String>, classes: &mut Vec<Class>, relations: &mut Vec<Relation>){
    let mut count_classes: i32 = 0;
    let mut count_relations: i32 = 0;

    let mut class_keywords: HashSet<String> = HashSet::new();
    class_keywords.insert(String::from("class"));
    class_keywords.insert(String::from("simpleclass"));
    class_keywords.insert(String::from("abstractclass"));
    class_keywords.insert(String::from("activeclass"));
    class_keywords.insert(String::from("varborderclass"));
    class_keywords.insert(String::from("dashedborderclass"));

    lines.reverse();

    loop {
        let opt = lines.pop();
        if !opt.is_some(){
            break;
        }
        let mut string = opt.unwrap();
        string.pop();
        let mut lower = string.to_lowercase();

        let mut split_lower = lower.split(":");
        let split_list_lower: Vec<&str> = split_lower.collect();

        let mut split_string = string.split(":");
        let split_list_string: Vec<&str> = split_string.collect();

        if split_list_lower.len() == 1{
            //Case: The header line was not a class
            //println!("Is no class: {}", string);




        }else{
            //Case: The header line was a class
            //println!("Is class: {}", string);

            let mut class_type: ClassType = ClassType::None;

            let tmp_string: &str = split_list_lower.get(0).unwrap();

            if tmp_string.starts_with("class") || tmp_string.starts_with("simpleclass"){
                class_type = ClassType::SimpleClass;
            }else if tmp_string.starts_with("abstractclass"){
                class_type = ClassType::AbstractClass;
            }else if tmp_string.starts_with("activeclass"){
                class_type = ClassType::ActiveClass;
            }else if tmp_string.starts_with("varborderclass"){
                class_type = ClassType::VarBorderClass;
            }else if tmp_string.starts_with("dashedbordertclass"){
                class_type = ClassType::DashedBorderClass;
            }else{
                println!("Error in class syntax: ClassType {} not found.", tmp_string);
            }

            let mut class_name: String = split_list_string.get(1).unwrap().to_string();

            let mut content_lines: Vec<String> = Vec::new();
            let mut content_decor: Vec<TextDecoration> = Vec::new();
            //let mut content: HashMap<String, TextDecoration> = HashMap::new();

            loop {
                let opt_inner = lines.pop();
                if !opt_inner.is_some(){
                    break;
                }
                let mut line_inner = opt_inner.unwrap();
                line_inner.pop();

                if line_inner.eq("--"){
                    content_lines.push(String::from(""));
                    content_decor.push(TextDecoration::HorizontalLine);
                    continue;
                }

                let mut field_visibility: Visibility = Visibility::None;
                let mut is_static: bool = false;
                let mut skip: usize = 0;

                if line_inner.starts_with("private "){
                    field_visibility = Visibility::Private;
                    skip = 8;
                }else if line_inner.starts_with("public "){
                    field_visibility = Visibility::Public;
                    skip = 7;
                }else if line_inner.starts_with("protected "){
                    field_visibility = Visibility::Protected;
                    skip = 10;
                }else if line_inner.starts_with("package "){
                    field_visibility = Visibility::Package;
                    skip = 8;
                }else if line_inner.starts_with("static "){
                    field_visibility = Visibility::Package;
                    skip = 7;
                    is_static = true;
                }

                line_inner = line_inner.chars().skip(skip).take(line_inner.len()-skip).collect();

                /*println!("left of line after first cut: {}", line_inner);
                println!("classType: {:?}", class_type);
                println!("className: {}", class_name);
                println!("field visibility: {:?}", field_visibility);*/

                if line_inner.starts_with("static "){
                    skip = 7;
                    is_static = true;

                    line_inner = line_inner.chars().skip(skip).take(line_inner.len()-skip).collect();
                }

                let mut vis_string: String = String::new();

                if field_visibility == Visibility::Public{
                    vis_string = String::from("+ ");
                }else if field_visibility == Visibility::Package{
                    vis_string = String::from("~ ");
                }else if field_visibility == Visibility::Protected{
                    vis_string = String::from("# ");
                }else if field_visibility == Visibility::Private{
                    vis_string = String::from("- ");
                }

                let mut total: String = vis_string + &line_inner;

                let mut field_decor: TextDecoration = TextDecoration::None;

                if is_static == true{
                    field_decor = TextDecoration::Underlined;
                }

                content_lines.push(total);
                content_decor.push(field_decor);
            }

            let mut class: Class = Class {border_width: 0, class_name, class_type, content_lines, content_decor};
            println!("Full struct for class: {:?}", class);

            classes.push(class);
        }
    }
}

fn read_file(vec: &mut Vec<String>){
    let path = "input.txt";
    /*let buffered = BufReader::new(file);

    for line in buffered.lines() {
        let l = line.unwrap();
        vec.push(l);
    }*/

    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut split = contents.split("\n");
    let list: Vec<&str> = split.collect();

    for string in list.iter(){
        vec.push(String::from_str(*string).unwrap());
    }

    //println!("With text:\n{}", contents);
}