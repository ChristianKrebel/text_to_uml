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


pub fn parse_model(lines: &[String]) -> Result<ModelContainer, ParseError> {

    //========== Test Implementation ==========
    let mut classes: Vec<Class> = Vec::new();
    let mut relations: Vec<Relation> = Vec::new();
    let mut content_lines: Vec<String> = Vec::new();
    let mut content_decor: Vec<TextDecoration> = Vec::new();
    content_lines.push(String::from("-"));
    content_lines.push(String::from("- Attribute"));
    content_decor.push(TextDecoration::None);
    content_decor.push(TextDecoration::None);
    let mut content_lines2: Vec<String> = Vec::new();
    let mut content_decor2: Vec<TextDecoration> = Vec::new();
    content_lines2.push(String::from("-"));
    content_lines2.push(String::from("- Attribute"));
    content_decor2.push(TextDecoration::None);
    content_decor2.push(TextDecoration::Underlined);
    let mut class: Class = Class
        {
            class_type: ClassType::SimpleClass,
            class_name: String::from("Klasse"),
            class_stereotype: String::from("<<interface>>"),
            border_width: 0,
            content_lines: content_lines,
            content_decor: content_decor
        };
    let mut class2: Class = Class
        {
            class_type: ClassType::SimpleClass,
            class_name: String::from("Klasse2"),
            class_stereotype: String::from("<<blub>>"),
            border_width: 0,
            content_lines: content_lines2,
            content_decor: content_decor2
        };
    classes.push(class);
    classes.push(class2);

    let mut relation: Relation = Relation
        {
            border_type: BorderType::Dashed,
            arrow_type: RelationArrow::DiamondFilled,
            from_class: String::from("Klasse"),
            from_class_card: String::from("n"),
            to_class: String::from("Klasse2"),
            to_class_card: String::from("*")
        };
    relations.push(relation);

    let cm: ClassModel = ClassModel {classes, relations};

    let objects: Vec<Object> = Vec::new();
    let links: Vec<Link> = Vec::new();
    let om: ObjectModel = ObjectModel {objects, links};

    let packages: Vec<Package> = Vec::new();
    let packageRelations: Vec<PackageRelation> = Vec::new();
    let pm: PackageModel = PackageModel {packages, relations: packageRelations};

    let system: System = System { system_name: "s".to_string()};
    let use_cases: Vec<UseCase> = Vec::new();
    let participants: Vec<Participant> = Vec::new();
    let useCaseRelations: Vec<UseCaseRelation> = Vec::new();
    let ucm: UseCaseModel = UseCaseModel {
        system,
        use_cases,
        participants,
        relations: useCaseRelations
    };

    let mc: ModelContainer = ModelContainer {
        model_type: ModelType::ClassModel,
        class_model: cm,
        object_model: om,
        package_model: pm,
        use_case_model: ucm
    };

    Ok(mc)
    //========================================
}