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
use std::process::exit;

use defines::*;


// Testing Modul. Delete this function and implement your own Hannes.
pub fn init(text_input: &str, is_raw: bool) -> Result<(Vec<Class>, Vec<Relation>), io::Error> {
    let mut lines = if is_raw { read_text(text_input)? } else { read_file(text_input)? };
    for line in lines.iter(){
        println!("lines: {}", line);
    }
    Ok(parse_lines(&mut lines))
}

fn parse_lines(lines: &mut Vec<String>) -> (Vec<Class>, Vec<Relation>) {

    let mut classes = Vec::new();
    let mut relations = Vec::new();

    lines.reverse();

    loop {
        let opt = lines.pop();
        if !opt.is_some(){
            break;
        }
        let mut string = opt.unwrap();
        string.pop();

        let lower = string.to_lowercase();
        let split_list_lower: Vec<&str> = lower.split(":").collect();
        let split_list_string: Vec<&str> = string.split(":").collect();

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
                println!("No RelationType fitting found. '{}'\nAborting...", relation_string);
                exit(-1);
            }

            //println!("Value of relation_type: {:?}", relation_type);

            let mut opt_classes = lines.pop();

            if !opt_classes.is_some() {
                println!("Error in relation syntax: Missing lines following class type definition1");
                break;
            }

            let mut classes_str = opt_classes.unwrap();
            classes_str.pop();

            if classes_str.is_empty() {
                println!("Error in relation syntax near '{}': Missing lines following class type definition\nAborting...", classes_str);
                exit(-1);
            }

            let classes_split = classes_str.split(",");
            let classes_split_list: Vec<&str> = classes_split.collect();

            if *(classes_split_list.get(0).unwrap()) == classes_str || classes_split_list.len() > 2{ // Invalid
                println!("Error in relation syntax near '{}': Please enter two classnames seperated by a comma\nAborting...", classes_str);
                exit(-1);
            }

            let from_class_name = classes_split_list.get(0).unwrap();
            let to_class_name = classes_split_list.get(1).unwrap();

            let opt_cards = lines.pop();

            if !opt_cards.is_some() {

                let relation: Relation = Relation {
                    arrow_type,
                    border_type: relation_border,
                    from_class: String::from_str(*from_class_name).unwrap(),
                    from_class_card: String::new(),
                    to_class: String::from_str(to_class_name).unwrap(),
                    to_class_card: String::new(),
                };

                relations.push(relation);
                continue;
            }

            let mut opt_cards_str = opt_cards.unwrap();
            opt_cards_str.pop();

            // No multiplicities given.
            if opt_cards_str.is_empty() {

                let mut relation: Relation = Relation {
                    arrow_type,
                    border_type: relation_border,
                    from_class: String::from_str(*from_class_name).unwrap(),
                    from_class_card: String::new(),
                    to_class: String::from_str(to_class_name).unwrap(),
                    to_class_card: String::new(),
                };


                relations.push(relation);
                continue;
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


            let mut opt_empty_line = lines.pop();

            if !opt_empty_line.is_some() {
                continue;
            }

            let mut empty_line_str = opt_empty_line.unwrap();
            empty_line_str.pop();

            //println!("empty_line: '{}'", empty_line_str);

            if opt_cards_str.is_empty() {
                continue;
            }

        }else{
            //Case: The header line was a class
            //println!("Is class: {}", string);

            /*
             * Class Type
             */
            let mut class_type: ClassType = ClassType::None;
            let mut class_stereotype: String = String::from("");

            let tmp_string: &str = split_list_lower.get(0).unwrap();

            if tmp_string.starts_with("class") || tmp_string.starts_with("simpleclass"){
                class_type = ClassType::SimpleClass;
            }else if tmp_string.starts_with("abstractclass"){
                class_type = ClassType::AbstractClass;
                class_stereotype = String::from("<<abstract>>");
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
            let class_name: String = split_list_string.get(1).unwrap().to_string();


            /*
             * Stereotypes
             */
            let mut opt_inner = lines.pop();
            if !opt_inner.is_some() {
                println!("Error in class syntax: Missing lines following class type definition");
                break;
            }

            let mut line_stereo = opt_inner.unwrap();

            if class_name.eq("MyList"){
                println!("{}", line_stereo);
            }

            if line_stereo.is_empty() { //No line following the class_name
                //println!("Error in class syntax: Missing lines following class type definition");
                let mut content_lines: Vec<String> = Vec::new();
                let mut content_decor: Vec<TextDecoration> = Vec::new();

                let mut class: Class = Class {border_width: 0, class_name, class_type, content_lines, content_decor, class_stereotype};
                println!("Full struct for class: {:?}", class);

                classes.push(class);

                continue;
            }

            //let mut line_stereo_before = String::from(line_stereo)
            //line_stereo.pop();

            if line_stereo.starts_with("<<") && line_stereo.ends_with(">>\r") && !class_stereotype.is_empty(){
                line_stereo.pop();
                class_stereotype = line_stereo;
            }else{
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


                if line_inner.starts_with("static "){
                    skip = 7;
                    is_static = true;

                    line_inner = line_inner.chars().skip(skip).take(line_inner.len()-skip).collect();
                }

                let mut split = line_inner.split(" ");
                let all_in_line: Vec<&str> = split.collect();

                let mut line_left = String::new();

                if all_in_line.len() > 1{
                    let strEnd: String = all_in_line.get(0).unwrap().to_string();
                    let mut strStart: String = line_inner.chars().skip(strEnd.len()+1).take(line_inner.len()-(strEnd.len()+1)).collect();
                    line_left = strStart + ": " + &strEnd;
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

                let mut total: String = vis_string + &line_left;

                let mut field_decor: TextDecoration = TextDecoration::None;

                if is_static == true{
                    field_decor = TextDecoration::Underlined;
                }

                content_lines.push(total);
                content_decor.push(field_decor);
            }

            let class: Class = Class {
                border_width: 0,
                class_name,
                class_type,
                content_lines,
                content_decor,
                class_stereotype
            };

            classes.push(class);
        }
    }

    (classes, relations)
}

fn read_file(filename: &str) -> Result<Vec<String>, io::Error> {
    use std::fs;
    let contents = fs::read_to_string(filename)?;
    Ok(contents.lines().map(|line| line.to_string()).collect())
}

fn read_text(text: &str) -> Result<Vec<String>, io::Error> {
    Ok(text.lines().map(|line| line.to_string()).collect())
}