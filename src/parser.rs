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
pub fn init(filename: &str, mut classes: &mut Vec<Class>, mut relations: &mut Vec<Relation>) -> String{

    let mut vec: Vec<String> = Vec::new();
    read_file(&mut vec, filename);

    parse_lines(&mut vec, &mut classes, &mut relations);


    for line in vec.iter(){
        println!("lines: {}", line);
    }

    let str = String::from("done reading file.");
    return str;
}

fn parse_lines(lines: &mut Vec<String>, classes: &mut Vec<Class>, relations: &mut Vec<Relation>){

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
            //if split_list_lower.get(0).unwrap()
            //Case: The header line was not a class, e.g.
            //println!("Is no class: {}", string);

            let mut arrow_type: RelationArrow = RelationArrow::None;
            let mut relation_string: &str = split_list_lower.get(0).unwrap();
            let mut relation_border: BorderType = BorderType::None;

            if relation_string == "assoc" || relation_string == "association" {
                arrow_type = RelationArrow::Arrow;
                relation_border = BorderType::Solid;
            }else if relation_string == "inherit" || relation_string == "inheritance"{
                arrow_type = RelationArrow::TriangleEmpty;
                relation_border = BorderType::Solid;
            }else if relation_string == "implement" || relation_string == "implementation"{
                arrow_type = RelationArrow::TriangleEmpty;
                relation_border = BorderType::Dashed;
            }else if relation_string == "depend" || relation_string == "dependency"{
                arrow_type = RelationArrow::Arrow;
                relation_border = BorderType::Dashed;
            }else if relation_string == "aggregate" || relation_string == "aggregation"{
                arrow_type = RelationArrow::DiamondEmpty;
                relation_border = BorderType::Solid;
            }else if relation_string == "composit" || relation_string == "composition"{
                arrow_type = RelationArrow::DiamondFilled;
                relation_border = BorderType::Solid;
            }else{
                println!("No RelationType fitting found.");
            }

            //println!("Value of relation_type: {:?}", relation_type);

            let mut opt_classes = lines.pop();

            if !opt_classes.is_some() {
                println!("Error in relation syntax: Missing lines following class type definition");
                break;
            }

            let mut classes_str = opt_classes.unwrap();
            classes_str.pop();

            if classes_str.is_empty() {
                println!("Error in relation syntax: Missing lines following class type definition");
                break;
            }

            let mut classes_split = classes_str.split(",");
            let classes_split_list: Vec<&str> = classes_split.collect();

            if *(classes_split_list.get(0).unwrap()) == classes_str{ // Invalid
                println!("Error in relation syntax: Please enter two classnames seperated by a comma");
                break;
            }



            let mut from_class_name = classes_split_list.get(0).unwrap();
            let mut to_class_name = classes_split_list.get(1).unwrap();



            let mut opt_cards = lines.pop();

            if !opt_cards.is_some() {
                println!("Error in relation syntax: Missing lines following class type definition");
                break;
            }

            let mut opt_cards_str = opt_cards.unwrap();
            opt_cards_str.pop();

            if opt_cards_str.is_empty() {
                println!("Error in relation syntax: Missing lines following class type definition");
                break;
            }


            let mut cards_split = opt_cards_str.split(",");
            let cards_split_list: Vec<&str> = cards_split.collect();

            if *(cards_split_list.get(0).unwrap()) == opt_cards_str{ // Invalid
                println!("Error in relation syntax: Please enter two classnames seperated by a comma");
                break;
            }

            let mut from_card = cards_split_list.get(0).unwrap();
            let mut to_card = cards_split_list.get(1).unwrap();

            let mut relation: Relation = Relation {arrow_type, border_type: relation_border, from_class: String::from_str(*from_class_name).unwrap(), from_class_card: String::from_str(from_card).unwrap(), to_class: String::from_str(to_class_name).unwrap(), to_class_card: String::from_str(to_card).unwrap()};
            println!("Full struct for relation: {:?}", relation);

            relations.push(relation);

        }else{
            //Case: The header line was a class
            //println!("Is class: {}", string);

            /*
             * Class Type
             */
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

            /*
             * Class Name
             */
            let mut class_name: String = split_list_string.get(1).unwrap().to_string();


            /*
             * Stereotypes
             */
            let mut opt_inner = lines.pop();
            if !opt_inner.is_some() {
                println!("Error in class syntax: Missing lines following class type definition");
                break;
            }

            let mut line_stereo = opt_inner.unwrap();

            if line_stereo.is_empty() {
                println!("Error in class syntax: Missing lines following class type definition");
                break;
            }

            line_stereo.pop();

            let mut class_stereotype: String;

            if line_stereo.starts_with("<<") && line_stereo.ends_with(">>") {
                class_stereotype = line_stereo;
            }else{
                class_stereotype = String::from("");
                lines.push(line_stereo);
            }


            /*
             * Content lines
             */
            let mut content_lines: Vec<String> = Vec::new();
            let mut content_decor: Vec<TextDecoration> = Vec::new();
            //let mut content: HashMap<String, TextDecoration> = HashMap::new();

            loop {
                opt_inner = lines.pop();
                if !opt_inner.is_some(){
                    //println!("Error in class syntax: Missing lines following class type / stereotype definition");
                    break;
                }
                let mut line_inner = opt_inner.unwrap();
                line_inner.pop();

                if line_inner == "" {
                    break;
                }

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

            let mut class: Class = Class {border_width: 0, class_name, class_type, content_lines, content_decor, class_stereotype};
            println!("Full struct for class: {:?}", class);

            classes.push(class);
        }
    }
}

fn read_file(vec: &mut Vec<String>, filename: &str){
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

    let mut someInt = 0;
    for string in list.iter(){
        let mut string: String = String::from_str(*string).unwrap();
        if string == "" && list.len()-1 == someInt{
            continue;
        }
        vec.push(string);

        someInt += 1;
    }


    //println!("With text:\n{}", contents);
}