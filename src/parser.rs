#![allow(dead_code)]
#[macro_use]

use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::io;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io::prelude::*;
use std::str::*;
use std::str;
use std::process::exit;
use std::fs;
use std::fmt;
use nom::*;
use nom;
use std::fmt::Debug;

use defines::*;



named!(model_type<&str, ModelType>,
    do_parse!(
        ws!(tag_s!("Model:")) >>
        model: alt!(
            value!(ModelType::ClassModel, tag_s!("Class"))     |
            value!(ModelType::ObjectModel, tag_s!("Object"))   |
            value!(ModelType::PackageModel, tag_s!("Package")) |
            value!(ModelType::UseCaseModel, tag_s!("UseCase"))
        ) >>
        (model)
    )
);

named!(model_end<&[u8], &str>,
    do_parse!(
        take_while!(is_ws) >>
        end_tag: value!("/Model", tag!(&b"/Model"[..])) >>
        (end_tag)
    )
);

named!(parse_till_ws<&[u8], &str>,
    do_parse!(
        take_while!(is_ws) >>
        vis: take_while!(is_not_ws) >>
        (str::from_utf8(vis).unwrap())
    )
);

named!(parse_till_newline<&[u8], &str>,
    do_parse!(
        take_while!(is_ws) >>
        vis: take_while!(is_not_newline) >>
        (str::from_utf8(vis).unwrap())
    )
);

named!(parse_till_gt<&[u8], &str>,
    do_parse!(
        take_while!(is_ws) >>
        vis: take_while!(is_not_gt) >>
        (str::from_utf8(vis).unwrap())
    )
);

named!(cd_stereotype<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"<<"[..]) >>
        stereotype: parse_till_gt >>
        tag!(&b">>"[..]) >>
        (format!("<<{}>>", stereotype))
    )
);

named!(cd_class_type<&[u8], (ClassType, String)>,
    do_parse!(
        take_while!(is_ws) >>
        class_type: alt!(
            value!(ClassType::SimpleClass, tag!(&b"Class"[..]))                   |
            value!(ClassType::AbstractClass, tag!(&b"AbstractClass"[..]))         |
            value!(ClassType::VarBorderClass, tag!(&b"VarBorderClass"[..]))       |
            value!(ClassType::DashedBorderClass, tag!(&b"DashedBorderClass"[..])) |
            value!(ClassType::ActiveClass, tag!(&b"ActiveClass"[..]))
        ) >>
        tag!(&b":"[..]) >>
        class_name: parse_till_newline >>
        ((class_type, String::from(class_name)))
    )
);

named!(cd_relation_type<&[u8], RelationType>,
    do_parse!(
        take_while!(is_ws) >>
        relation_type: alt!(
            value!(RelationType::Association, tag!(&b"Association"[..]))       |
            value!(RelationType::Inheritance, tag!(&b"Inheritance"[..]))       |
            value!(RelationType::Implementation, tag!(&b"Implementation"[..])) |
            value!(RelationType::Dependency, tag!(&b"Dependency"[..]))         |
            value!(RelationType::Aggregation, tag!(&b"Aggregation"[..]))       |
            value!(RelationType::Composition, tag!(&b"Composition"[..]))
        ) >>
        (relation_type)
    )
);

named!(cd_relation_direction<&[u8], (String, String)>,
    do_parse!(
        take_while!(is_ws) >>
        rel_from: map!(map!(take_while!(is_not_comma), str::from_utf8), std::result::Result::unwrap) >>
        tag!(&b","[..]) >>
        rel_to: parse_till_newline >>
        ((String::from(rel_from), String::from(rel_to)))
    )
);

named!(cd_relation_cardinality<&[u8], (String, String)>,
    do_parse!(
        tag!(&b"\n"[..]) >>
        not!(tag!(&b"\n"[..])) >>
        card_from: map!(map!(take_while!(is_not_comma), str::from_utf8), std::result::Result::unwrap) >>
        tag!(&b","[..]) >>
        card_to: parse_till_newline >>
        ((String::from(card_from), String::from(card_to)))
    )
);

named!(cd_relation<&[u8], Relation>,
    do_parse!(
        rel_type: map!(cd_relation_type, get_fitting_relation_type) >>
        direction: cd_relation_direction >>
        cardinalities: map!(opt!(cd_relation_cardinality), get_fitting_relation_cardinality) >>
        (Relation { border_type: rel_type.0, arrow_type: rel_type.1, from_class: direction.0, to_class: direction.1, from_class_card: cardinalities.0, to_class_card: cardinalities.1 })
    )
);



named!(cd_visibility<&[u8], &str>,
    do_parse!(
        take_while!(is_ws) >>
        vis: alt!(
            value!("+", tag!(&b"public"[..]))    |
            value!("#", tag!(&b"protected"[..])) |
            value!("~", tag!(&b"package"[..]))   |
            value!("-", tag!(&b"private"[..]))
        ) >>
        (vis)
    )
);

named!(cd_static_modifier<&[u8], TextDecoration>,
    ws!(alt!(
        tag!(&b"static"[..]) => { |_| TextDecoration::Underlined }
    ))
);

named!(cd_abstract_modifier<&[u8], TextDecoration>,
    do_parse!(
        take_while!(is_ws) >>
        modif: value!(TextDecoration::Italic, tag!(&b"abstract"[..])) >>
        (modif)
    )
);

named!(cd_variable_pair<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        data_type: parse_till_ws >>
        var_name: parse_till_ws >>
        (format!("{}: {}", var_name, data_type))
    )
);

named!(cd_params_variable_pair<&[u8], String>,
    do_parse!(
        not!(tag!(&b")"[..])) >>
        take_while!(is_ws) >>
        data_type: parse_till_ws >>
        tag!(&b" "[..]) >>
        var_name: map!(map!(take_while!(is_not_comma_and_cb), str::from_utf8), std::result::Result::unwrap) >>
        (format!("{}: {}", var_name, data_type))
    )
);

named!(cd_method_pair<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        data_type: parse_till_ws >>
        var_name: map!(map!(take_while!(is_not_ob_and_newline), str::from_utf8), std::result::Result::unwrap) >>
        tag!(&b"("[..]) >>
        params: opt!(separated_list!(tag!(&b", "[..]), cd_params_variable_pair)) >>
        tag!(&b")"[..]) >>
        (format!("{}({}): {}", var_name, if params.is_some() {params.unwrap().join(", ")} else {String::from("")}, data_type))
    )
);

//--
named!(cd_horizontal_line<&[u8], Line>,
    do_parse!(
        take_while!(is_ws) >>
        hor_line: value!(TextDecoration::HorizontalLine, tag!(&b"--"[..])) >>
        ( Line{ content: String::from(""), decor: hor_line } )
    )
);

//public static boolean someName3
named!(cd_member<&[u8], Line>,
    do_parse!(
        take_while!(is_ws) >>
        vis: map!(opt!(cd_visibility), get_fitting_visibility) >>
        decor: map!(opt!(cd_static_modifier), get_fitting_decor) >>
        not!(pair!(take_while!(is_ws), tag!(&b"abstract"[..]))) >>
        variable: cd_variable_pair >>
        ( Line{ content: format!("{} {}", vis, variable), decor } )
    )
);

//public abstract void main()
named!(cd_method<&[u8], Line>,
    do_parse!(
        take_while!(is_ws) >>
        vis: map!(opt!(cd_visibility), get_fitting_visibility) >>
        decor: map!(opt!(alt!(
            cd_static_modifier   |
            cd_abstract_modifier
        )), get_fitting_decor) >>
        variable: cd_method_pair >>
        ( Line{ content: format!("{}{}", vis, variable), decor } )
    )
);

named!(cd_line<&[u8], Line>,
    do_parse!(
        line: alt_complete!(
            cd_horizontal_line |
            cd_method          |
            cd_member
        ) >>
        (line)
    )
);

named!(cd_class<&[u8], Class>,
    do_parse!(
        class_type: cd_class_type >>
        stereotype: map!(opt!(cd_stereotype), get_fitting_stereotype) >>
        lines: many_till!(cd_line, alt!(tag!(&b"\n\n"[..]) | tag!(&b"\n/Model"[..]))) >>
        (Class { class_type: class_type.0, class_name: class_type.1, border_width: 1, class_stereotype: stereotype, lines: lines.0 })
    )
);


named!(cd_class_model<&[u8], ClassModel>,
    do_parse!(
        classes: many1!(cd_class) >>
        relations: many_till!(cd_relation, pair!(take_while!(is_ws), tag!(&b"/Model"[..]))) >>
        (ClassModel { classes, relations: relations.0 })
    )
);



// ====== Object Model ======



named!(obj_object_name<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"Object"[..]) >>
        has_name: opt!(tag!(&b":"[..])) >>
        obj_name: cond!(has_name.is_some(), map!(parse_till_newline, String::from)) >>
        ( if obj_name.is_some() {obj_name.unwrap()} else {String::from("")} )
    )
);

named!(obj_object_title<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        obj_disp_name: map!(map!(take_while!(is_not_colon), str::from_utf8), std::result::Result::unwrap) >>
        obj_disp_class: parse_till_newline >>
        ( format!("{} {}", String::from(obj_disp_name), String::from(obj_disp_class)) )
    )
);

//<AttributName>[:<AttributTyp>] <AttributInhalt>
named!(obj_line<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        attrib_name_type: map!(parse_till_ws, get_fitting_object_line_name_type) >>
        attrib_content: parse_till_newline >>
        ( format!("{}{} = {}", attrib_name_type.0, attrib_name_type.1, attrib_content) )
    )
);

named!(obj_object<&[u8], Object>,
    do_parse!(
        obj_name: obj_object_name >>
        obj_title: obj_object_title >>
        lines: many_till!(obj_line, alt!(tag!(&b"\n\n"[..]) | tag!(&b"\n/Model"[..]))) >>
        (Object { object_intern_name: obj_name, object_title: obj_title, content_lines: lines.0 })
    )
);

named!(obj_link<&[u8], Link>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"Link"[..]) >>
        link_name: map!(opt!(map!(map!(take_while!(is_not_newline), str::from_utf8), std::result::Result::unwrap)), get_fitting_link_name) >>
        link_direction: cd_relation_direction >>
        link_roles: map!(opt!(cd_relation_cardinality), get_fitting_link_roles) >>
        ( Link { link_name, from_object: link_direction.0, to_object: link_direction.1, from_object_role: link_roles.0, to_object_role: link_roles.1 } )
    )
);


named!(obj_object_model<&[u8], ObjectModel>,
    do_parse!(
        objects: many1!(obj_object) >>
        links: many_till!(obj_link, pair!(take_while!(is_ws), tag!(&b"/Model"[..]))) >>
        (ObjectModel { objects, links: links.0 })
    )
);




// ====== Package Model ======

named!(pack_relation_type<&[u8], PackageRelName>,
    do_parse!(
        take_while!(is_ws) >>
        relation_type: alt!(
            value!(PackageRelName::Import, tag!(&b"Import"[..]))       |
            value!(PackageRelName::Access, tag!(&b"Access"[..]))       |
            value!(PackageRelName::Merge, tag!(&b"Merge"[..]))
        ) >>
        (relation_type)
    )
);

named!(pack_package_name<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"Package:"[..]) >>
        package_name: parse_till_newline >>
        ( String::from(package_name) )
    )
);

named!(pack_sub_packages<&[u8], Vec<String>>,
    do_parse!(
        tag!(&b"\n"[..]) >>
        not!(tag!(&b"\n"[..])) >>
        sub_packages: separated_list!(tag!(&b","[..]), map!(map!(map!(take_while!(is_not_comma_and_newline), str::from_utf8), std::result::Result::unwrap), String::from)) >>
        ( sub_packages )
    )
);

named!(pack_package<&[u8], Package>,
    do_parse!(
        package_name: pack_package_name >>
        sub_packages: opt!(pack_sub_packages) >>
        ( Package { package_name, inner_packages: sub_packages } )
    )
);

named!(pack_relation<&[u8], PackageRelation>,
    do_parse!(
        relation_type: pack_relation_type >>
        direction: cd_relation_direction >>
        ( PackageRelation { package_rel_name: relation_type, from_package: direction.0, to_package: direction.1 } )
    )
);

named!(pack_package_model<&[u8], PackageModel>,
    do_parse!(
        packages: many1!(pack_package) >>
        relations: many_till!(pack_relation, pair!(take_while!(is_ws), tag!(&b"/Model"[..]))) >>
        (PackageModel { packages, relations: relations.0 })
    )
);



// ====== UseCase Model ======


named!(uc_system<&[u8], System>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"System:"[..]) >>
        system_name: parse_till_newline >>
        ( System { name: String::from(system_name) } )
    )
);

named!(uc_actor<&[u8], Actor>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"Actor:"[..]) >>
        actor_name_intern: map!(parse_till_newline, String::from) >>
        actor_name_display: opt!(tuple!(tag!(&b"\n"[..]), not!(tag!(&b"\n"[..])), map!(parse_till_newline, String::from))) >>
        ( Actor { name_intern: actor_name_intern.clone(), name_display: if actor_name_display.is_some() {actor_name_display.unwrap().2} else {actor_name_intern} } )
    )
);

named!(uc_actor_actor_relation<&[u8], ActorActorRelation>,
    do_parse!(
        take_while!(is_ws) >>
        alt!(tag!(&b"Generalization"[..]) |
             tag!(&b"Generalisation"[..])) >>
        tag!(&b"\n"[..]) >>
        direction: cd_relation_direction >>
        ( ActorActorRelation { border_type: BorderType::Solid, arrow_type: RelationArrow::TriangleEmpty, from_actor: direction.0, to_actor: direction.1 } )
    )
);

named!(uc_use_case<&[u8], UseCase>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"UseCase:"[..]) >>
        uc_name_intern: map!(parse_till_newline, String::from) >>
        uc_name_display: opt!(tuple!(tag!(&b"\n"[..]), not!(tag!(&b"\n"[..])), map!(parse_till_newline, String::from))) >>
        ( UseCase { name_intern: uc_name_intern.clone(), name_display: if uc_name_display.is_some() {uc_name_display.unwrap().2} else {uc_name_intern} } )
    )
);


named!(uc_use_case_relation_type<&[u8], UseCaseRelationType>,
    do_parse!(
        take_while!(is_ws) >>
        relation_type: alt!(
            value!(UseCaseRelationType::Include, tag!(&b"Include"[..]))              |
            value!(UseCaseRelationType::Extend, tag!(&b"Extend"[..]))                |
            value!(UseCaseRelationType::Generalize, alt!(tag!(&b"Generalize"[..])  |
                                                         tag!(&b"Generalise"[..])))
        ) >>
        (relation_type)
    )
);

named!(uc_use_case_use_case_relation<&[u8], UseCaseUseCaseRelation>,
    do_parse!(
        take_while!(is_ws) >>
        relation_type: uc_use_case_relation_type >>
        direction: cd_relation_direction >>
        note: opt!(tuple!(tag!(&b"\n"[..]), not!(tag!(&b"\n"[..])), map!(parse_till_newline, String::from))) >>
        ( UseCaseUseCaseRelation { border_arrow_type: get_fitting_uc_relation_type(&relation_type), relation_type: relation_type, from_use_case: direction.0, to_use_case: direction.1, note: if note.is_some() {Some(note.unwrap().2)} else {None} } )
    )
);

named!(uc_actor_use_case_relation<&[u8], ActorUseCaseRelation>,
    do_parse!(
        take_while!(is_ws) >>
        tag!(&b"Association\n"[..]) >>
        direction: cd_relation_direction >>
        ( ActorUseCaseRelation { border_type: BorderType::Solid, arrow_type: RelationArrow::None, from_actor: direction.0, to_use_case: direction.1 } )
    )
);


named!(uc_use_case_model<&[u8], UseCaseModel>,
    do_parse!(
        system: uc_system >>
        actors: many1!(uc_actor) >>
        aa_relation: many0!(uc_actor_actor_relation) >>
        use_cases: many1!(uc_use_case) >>
        ucuc_relation: many0!(uc_use_case_use_case_relation) >>
        auc_relation: many_till!(uc_actor_use_case_relation, pair!(take_while!(is_ws), tag!(&b"/Model"[..]))) >>
        (UseCaseModel { system, use_cases, actors, actor_actor_relations: aa_relation, use_case_use_case_relations: ucuc_relation, actor_use_case_relations: auc_relation.0 })
    )
);



// ====== Test Parser ======

named!(test<&[u8], Vec<String>>,
    do_parse!(
        ab_s: many_till!(map!(map!(map!(tag!(&b"ab"[..]), str::from_utf8), std::result::Result::unwrap), String::from), pair!(take_while!(is_ws), tag!(&b"z"[..]))) >>
        (ab_s.0)
    )
);



pub fn parse_model(lines: &[String]) -> Result<ModelContainer, ParseError> {


    let model_type_str = &*lines[0];

    let m_type = match model_type(model_type_str) {
        Ok(val) => val.1,
        Err(err) => {
            println!("Encountered error while parsing: {}", err);
            return Err(ParseError::InvalidModelError);
        }
    };

    println!("Using model type: {:?}", m_type);

    let mut all_lines = String::from("");
    let mut count = 0;
    for (i, line) in lines.iter().enumerate() {
        if count == 0{
            count = count + 1;
            continue;

        } else if count == 1{
            all_lines = format!("{}", *line);
        } else {
            all_lines = format!("{}\n{}", all_lines, *line);
        }

        count = count + 1;
    }

    let mut mc = match m_type {
        ModelType::ClassModel => {
            let class_model = match cd_class_model(all_lines.as_bytes()){
                Ok(val) => val.1,
                Err(err) => {
                    println!("Encountered error while parsing: {}", err);
                    return Err(ParseError::ParseError);
                }
            };

            ModelContainer {model_type: ModelType::ClassModel, class_model: Some(class_model), object_model: None, package_model: None, use_case_model: None}
        },
        ModelType::ObjectModel => {
            let object_model = match obj_object_model(all_lines.as_bytes()){
                Ok(val) => val.1,
                Err(err) => {
                    println!("Encountered error while parsing: {}", err);
                    return Err(ParseError::ParseError);
                }
            };

            ModelContainer {model_type: ModelType::ObjectModel, class_model: None, object_model: Some(object_model), package_model: None, use_case_model: None}
        },
        ModelType::PackageModel => {
            let package_model = match pack_package_model(all_lines.as_bytes()){
                Ok(val) => val.1,
                Err(err) => {
                    println!("Encountered error while parsing: {}", err);
                    return Err(ParseError::ParseError);
                }
            };

            ModelContainer {model_type: ModelType::ObjectModel, class_model: None, object_model: None, package_model: Some(package_model), use_case_model: None}
        },
        ModelType::UseCaseModel => {
            let uc_model = match uc_use_case_model(all_lines.as_bytes()){
                Ok(val) => val.1,
                Err(err) => {
                    println!("Encountered error while parsing: {}", err);
                    return Err(ParseError::ParseError);
                }
            };

            ModelContainer {model_type: ModelType::ObjectModel, class_model: None, object_model: None, package_model: None, use_case_model: Some(uc_model)}
        },
        ModelType::None => return Err(ParseError::InvalidModelError)
    };



    Ok(mc)
}


// =============================================
//     Helper functions to debug nom Results
// =============================================

fn confirm_result<T: Debug>(res: &Result<(&str, T), nom::Err<&str>>, message_on_failure: &str) -> bool{
    match res{
        Ok(v) => {
            println!("Parsed successfully: {:?}", v);
            return true;
        },
        Err(e) => {
            match e{
                nom::Err::Incomplete(n) => println!("Incomplete: {:?}", n),
                nom::Err::Error(e) => {
                    //println!("Error while reading Tags: ErrorKind: {}", e.into_error_kind().description());
                    println!("Error while parsing: {}", message_on_failure);
                },
                nom::Err::Failure(e) => println!("Failure")
            }
            return false;
        }
    }
}

fn confirm_result_byte<T: Debug>(res: &Result<(&[u8], T), nom::Err<&[u8]>>, message_on_failure: &str) -> bool{
    match res{
        Ok(v) => {
            let (left, parsed) = v;
            println!("Parsed successfully: {:?}", str::from_utf8(left));
            return true;
        },
        Err(e) => {
            match e{
                nom::Err::Incomplete(n) => println!("Incomplete: {:?}", n),
                nom::Err::Error(e) => {
                    //println!("Error while reading Tags: ErrorKind: {}", e.into_error_kind().description());
                    println!("Error while parsing: {}", message_on_failure);
                },
                nom::Err::Failure(e) => println!("Failure")
            }
            return false;
        }
    }
}

// =============================================



// ================================
//     Mapped functions in nom
// ================================

fn is_ws(c: u8) -> bool {
    return nom::is_space(c) || c == b'\n';
}

fn is_newline(c: u8) -> bool {
    return c == b'\n';
}

fn is_not_ws(c: u8) -> bool {
    return !nom::is_space(c) && c != b'\n';
}

fn is_not_newline(c: u8) -> bool {
    return c != b'\n';
}

fn is_not_gt(c: u8) -> bool {
    return c != b'>';
}

fn is_not_comma(c: u8) -> bool {
    return c != b',';
}

fn is_not_comma_and_newline(c: u8) -> bool {
    return c != b',' && c != b'\n';
}

fn is_not_comma_and_cb(c: u8) -> bool {
    return c != b',' && c != b')';
}

fn is_not_colon(c: u8) -> bool {
    return c != b':';
}

fn is_not_ob_and_newline(c: u8) -> bool {
    return c != b'(' && c != b'\n';
}

fn get_fitting_decor(line_decor: Option<TextDecoration>) -> TextDecoration{
    return match line_decor{
        Some(t) => {
            return t;
        },
        _ => {
            return TextDecoration::None;
        }
    }
}

fn get_fitting_visibility(vis: Option<&str>) -> &str{
    return match vis{
        Some(t) => t,
        _ => "~"
    }
}

fn get_fitting_stereotype(vis: Option<String>) -> String{
    return match vis{
        Some(t) => t,
        _ => String::from("")
    }
}

fn get_fitting_relation_cardinality(vis: Option<(String, String)>) -> (String, String){
    return match vis{
        Some(t) => t,
        _ => (String::from(""), String::from(""))
    }
}

fn get_fitting_relation_type(rel_type: RelationType) -> (BorderType, RelationArrow){
    return match rel_type{
        RelationType::Association => (BorderType::Solid, RelationArrow::Arrow),
        RelationType::Inheritance => (BorderType::Solid, RelationArrow::TriangleEmpty),
        RelationType::Implementation => (BorderType::Dashed, RelationArrow::TriangleEmpty),
        RelationType::Dependency => (BorderType::Dashed, RelationArrow::Arrow),
        RelationType::Aggregation => (BorderType::Solid, RelationArrow::DiamondEmpty),
        RelationType::Composition => (BorderType::Solid, RelationArrow::DiamondFilled),
        RelationType::None => (BorderType::Solid, RelationArrow::Arrow)
    }
}

fn get_fitting_link_name(vis: Option<&str>) -> String{
    return match vis{
        Some(t) => {
            if t.starts_with(":"){
                return t.chars().skip(1).take(t.len()-1).collect();
            }else{
                return String::from(t);
            }
        },
        _ => String::from("")
    }
}

fn get_fitting_link_roles(vis: Option<(String, String)>) -> (String, String){
    return match vis{
        Some(t) => t,
        _ => (String::from(""), String::from(""))
    }
}

fn get_fitting_object_line_name_type(name_type: &str) -> (String, String){
    let mut split = name_type.split(":");


    let attr_name = split.next().unwrap();

    let attr_type_opt = split.next();


    if !attr_type_opt.is_some() {
        return (String::from(name_type), String::from(""));
    }else{
        let attr_type = format!(": {}", String::from(attr_type_opt.unwrap()));

        return (String::from(attr_name), attr_type);
    }
}

fn get_fitting_object_name(vis: Option<&str>) -> String{
    return match vis{
        Some(t) => String::from(t),
        _ => String::from("")
    }
}

fn get_fitting_uc_relation_type(rel_type: &UseCaseRelationType) -> (BorderType, RelationArrow){
    return match rel_type{
        UseCaseRelationType::Include => (BorderType::Dashed, RelationArrow::Arrow),
        UseCaseRelationType::Extend => (BorderType::Dashed, RelationArrow::Arrow),
        UseCaseRelationType::Generalize => (BorderType::Solid, RelationArrow::TriangleEmpty)
    }
}

// ===============================








// | .================================================================. |
// | |                                                                | |
// | |                           TEST CODE                            | |
// | |                                                                | |
// | '================================================================' |


#[test]
fn test_test1(){
    assert_eq!(test(&b"abababab  z"[..]), Ok((&b""[..], vec![String::from("ab"), String::from("ab"), String::from("ab"), String::from("ab")])));
}

// +------------------------------------+
// |          Test model type           |
// +------------------------------------+



#[test]
fn test_model_type1(){
    assert_eq!(model_type("Model:Class"), Ok(("", ModelType::ClassModel)));
}

#[test]
fn test_model_type2(){
    assert_eq!(model_type("        Model:   Object"), Ok(("", ModelType::ObjectModel)));
}

#[test]
fn test_model_type3(){
    assert_eq!(model_type(" Model: Package"), Ok(("", ModelType::PackageModel)));
}

#[test]
fn test_model_type4(){
    assert_eq!(model_type("Model: UseCase"), Ok(("", ModelType::UseCaseModel)));
}


#[test]
fn test_parse_until_ws1(){
    assert_eq!(parse_till_ws(&b"boolean someName2"[..]), Ok((&b" someName2"[..], "boolean")));
}

#[test]
fn test_parse_until_ws2(){
    assert_eq!(parse_till_ws(&b"     boolean someName2"[..]), Ok((&b" someName2"[..], "boolean")));
}


#[test]
fn test_stereotype1(){
    assert_eq!(cd_stereotype(&b" <<abstract>> "[..]), Ok((&b" "[..], String::from("<<abstract>>"))));
}



// +------------------------------------+
// |         Test CD Visibility         |
// +------------------------------------+


#[test]
fn test_cd_visibility1(){
    assert_eq!(cd_visibility(&b"public"[..]), Ok((&b""[..], "+")));
}

#[test]
fn test_cd_visibility2(){
    assert_eq!(cd_visibility(&b"  public "[..]), Ok((&b" "[..], "+")));
}

#[test]
fn test_cd_visibility3(){
    let vis = match cd_visibility(&b"sumTingWong"[..]){
        Ok(val) => val,
        Err(err) => {
            assert!(true);
            return;
        }
    };

    assert!(false, "Visibility parsing unsuccessful: {:?}", vis);
}

#[test]
fn test_cd_visibility4(){
    assert_eq!(cd_visibility(&b"  private  "[..]), Ok((&b"  "[..], "-")));
}


// +------------------------------------+
// |      Test individual parsers       |
// +------------------------------------+

#[test]
fn test_parse_till_gt(){
    assert_eq!(parse_till_gt(&b"  sumWeirdStr<>sdsd "[..]), Ok((&b">sdsd "[..], "sumWeirdStr<")));
}



// +------------------------------------+
// |   Test CD member name/type pair    |
// +------------------------------------+

#[test]
fn test_cd_variable_pair(){
    assert_eq!(cd_variable_pair(&b"  boolean someVarName "[..]), Ok((&b" "[..], String::from("someVarName: boolean"))));
}

// +------------------------------------+
// |           Test CD member           |
// +------------------------------------+

#[test]
fn test_cd_member1(){
    let line: Line = match cd_member(&b"  public static boolean someName "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "+ someName: boolean");
    assert_eq!(line.decor, TextDecoration::Underlined);
}

#[test]
fn test_cd_member2(){
    let line: Line = match cd_member(&b"  private SumLongClass someOtherName "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "- someOtherName: SumLongClass");
    assert_eq!(line.decor, TextDecoration::None);
}

#[test]
fn test_cd_member3(){
    let line: Line = match cd_member(&b"  protected static int number "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "# number: int");
    assert_eq!(line.decor, TextDecoration::Underlined);
}

#[test]
fn test_cd_member4(){
    let line: Line = match cd_member(&b"  int number "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "~ number: int");
    assert_eq!(line.decor, TextDecoration::None);
}


// +------------------------------------+
// |           Test CD Method           |
// +------------------------------------+


#[test]
fn test_cd_method1(){
    let line: Line = match cd_method(&b"  public abstract void main()\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "+ main(): void");
    assert_eq!(line.decor, TextDecoration::Italic);
}

#[test]
fn test_cd_method2(){
    let line: Line = match cd_method(&b"  static Boolean someFunc()\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "~ someFunc(): Boolean");
    assert_eq!(line.decor, TextDecoration::Underlined);
}

#[test]
fn test_cd_method3(){
    let line: Line = match cd_method(&b" public abstract void shoutName()\n\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "+ shoutName(): void");
    assert_eq!(line.decor, TextDecoration::Italic);
}


#[test]
fn test_cd_method4(){
    let line: Line = match cd_method(&b"\npublic abstract void shoutName()\n\nClass:Chinese\n--\n"[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "+ shoutName(): void");
    assert_eq!(line.decor, TextDecoration::Italic);
}

#[test]
fn test_cd_method_with_params(){
    let line: Line = match cd_method(&b" public abstract void shoutName(int amount)\n\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "+ shoutName(amount: int): void");
    assert_eq!(line.decor, TextDecoration::Italic);
}

#[test]
fn test_cd_method_with_multiple_params(){
    let line: Line = match cd_method(&b" public abstract void shoutName(int amount, float volume)\n\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "+ shoutName(amount: int, volume: float): void");
    assert_eq!(line.decor, TextDecoration::Italic);
}

// +------------------------------------+
// |   Test CD Method name/type/params  |
// +------------------------------------+

#[test]
fn test_cd_method_pair_params(){
    let line: String = match cd_method_pair(&b" void shoutName(int amount)\n\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line, " shoutName(amount: int): void");
}



// +------------------------------------+
// |         Test CD full line          |
// +------------------------------------+

#[test]
fn test_cd_line1(){
    let line: Line = match cd_line(&b"  static Boolean someFunc()\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "Line parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(line.content, "~ someFunc(): Boolean");
    assert_eq!(line.decor, TextDecoration::Underlined);
}


// +------------------------------------+
// |         Test CD Class Type         |
// +------------------------------------+

#[test]
fn test_cd_class_type1(){
    let class_def: (ClassType, String) = match cd_class_type(&b"Class:SomeThing\n "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "ClassType parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(class_def.0, ClassType::SimpleClass);
    assert_eq!(class_def.1, String::from("SomeThing"));
}

#[test]
fn test_cd_class_type2(){
    let class_def: (ClassType, String) = match cd_class_type(&b"Class:Asdf\n<<interface>> "[..]){
        Ok(val) => val.1,
        Err(err) => {
            assert!(false, "ClassType parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(class_def.0, ClassType::SimpleClass);
    assert_eq!(class_def.1, String::from("Asdf"));
}



// +------------------------------------+
// |        Test CD full Classes        |
// +------------------------------------+

#[test]
fn test_cd_class_complete1(){
    let class: Class = match cd_class(&b" Class:TolleKlasse\nprivate abstract void enterText()\n\n "[..]){
        Ok(val) => {
            println!("Found class: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Class parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(class.class_type, ClassType::SimpleClass);
    assert_eq!(class.class_name, String::from("TolleKlasse"));
    assert_eq!(class.class_stereotype, String::from(""));
}

#[test]
fn test_cd_class_complete2(){
    let class: Class = match cd_class(&b" Class:TolleKlasse\n<<interfacebook>>\n--\n/Model"[..]){
        Ok(val) => {
            println!("Found class: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Class parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(class.class_type, ClassType::SimpleClass);
    assert_eq!(class.class_name, String::from("TolleKlasse"));
    assert_eq!(class.class_stereotype, String::from("<<interfacebook>>"));
}


#[test]
fn test_cd_class_complete3(){
    let class: Class = match cd_class(&b" Class:Person\n<<abstract>>\n--\nprotected String name\n--\npublic static void shoutName()\n\nClass:Chinese\n--\nprotected String name\n--\npublic void shoutName()\n\nInheritance\nChinese,Person\n1,1\n/Model "[..]){
        Ok(val) => {
            println!("Found class: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Class parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(class.class_type, ClassType::SimpleClass);
    assert_eq!(class.class_name, String::from("Person"));
    assert_eq!(class.class_stereotype, String::from("<<abstract>>"));
}



// +------------------------------------+
// |       Test CD full Relations       |
// +------------------------------------+


#[test]
fn test_cd_relation_complete_with_card(){
    let relation: Relation = match cd_relation(&b"Association\nA,B\n1..n,1\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(relation.border_type, BorderType::Solid);
    assert_eq!(relation.arrow_type, RelationArrow::Arrow);
    assert_eq!(relation.from_class, String::from("A"));
    assert_eq!(relation.to_class, String::from("B"));
    assert_eq!(relation.from_class_card, String::from("1..n"));
    assert_eq!(relation.to_class_card, String::from("1"));
}

#[test]
fn test_cd_relation_complete_no_card(){
    let relation: Relation = match cd_relation(&b"Association\nA,B\n\n/Model"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(relation.border_type, BorderType::Solid);
    assert_eq!(relation.arrow_type, RelationArrow::Arrow);
    assert_eq!(relation.from_class, String::from("A"));
    assert_eq!(relation.to_class, String::from("B"));
    assert_eq!(relation.from_class_card, String::from(""));
    assert_eq!(relation.to_class_card, String::from(""));
}


// +------------------------------------+
// |         Test CD full model         |
// +------------------------------------+

#[test]
fn test_cd_class_model_complete1(){
    let cm: ClassModel = match cd_class_model(&b"Class:Person\n<<abstract>>\n--\nprotected String name\n--\npublic abstract void shoutName()\n\nClass:Chinese\n--\nprotected String name\n--\npublic void shoutName()\n\nInheritance\nChinese,Person\n1,1\n/Model "[..]){
        Ok(val) => {
            println!("Found class model: {:?}", val.1);
            val.1
        },
        Err(err) => {
            match &err{
                Err::Error(c) => {
                    println!("Was error: {:?}", c.clone().into_error_kind());

                },
                Err::Failure(c) => {println!("Was failure: {:?}", c)},
                Err::Incomplete(n) => {println!("Was incomplete: {:?}", n)},
            }



            assert!(false, "Class Model parsing unsuccessful: {}", err);
            return;
        }
    };

    let classes = cm.classes;
    for class in &classes {
        println!("Class: {:?}", class);
    }

    let relations = cm.relations;
    for relation in &relations {
        println!("Relation: {:?}", relation);
    }

//    assert!(false);
}




// +------------------------------------+
// |        Test OBJ Object name        |
// +------------------------------------+

#[test]
fn test_obj_object_name(){
    assert_eq!(obj_object_name(&b" Object\n "[..]), Ok((&b"\n "[..], String::from(""))));
}

#[test]
fn test_obj_object_name2(){
    assert_eq!(obj_object_name(&b" Object:someName\n "[..]), Ok((&b"\n "[..], String::from("someName"))));
}




// +------------------------------------+
// |        Test OBJ Object title       |
// +------------------------------------+

#[test]
fn test_obj_object_title(){
    assert_eq!(obj_object_title(&b" lieblingsgrieche:Restaurant\n "[..]), Ok((&b"\n "[..], String::from("lieblingsgrieche :Restaurant"))));
}




// +------------------------------------+
// |        Test OBJ Object line        |
// +------------------------------------+

#[test]
fn test_obj_line(){
    assert_eq!(obj_line(&b" kategorie:Sterne 3\n "[..]), Ok((&b"\n "[..], String::from("kategorie: Sterne = 3"))));
}

#[test]
fn test_obj_line2(){
    assert_eq!(obj_line(&b" name \"Platon\"\n "[..]), Ok((&b"\n "[..], String::from("name = \"Platon\""))));
}



// +------------------------------------+
// |        Test OBJ Object link        |
// +------------------------------------+

#[test]
fn test_obj_link(){
    let link = match obj_link(&b" Link\nk1,lg\n+Arbeitnehmer,+Arbeitgeber\n\n "[..]){
        Ok(val) => {
            println!("Found link: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(link.link_name, String::from(""));
    assert_eq!(link.from_object, String::from("k1"));
    assert_eq!(link.to_object, String::from("lg"));
    assert_eq!(link.from_object_role, String::from("+Arbeitnehmer"));
    assert_eq!(link.to_object_role, String::from("+Arbeitgeber"));
}


#[test]
fn test_obj_link2(){
    let link = match obj_link(&b" Link:bedient\nk1,maren\n\n "[..]){
        Ok(val) => {
            println!("Found link: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(link.link_name, String::from("bedient"));
    assert_eq!(link.from_object, String::from("k1"));
    assert_eq!(link.to_object, String::from("maren"));
    assert_eq!(link.from_object_role, String::from(""));
    assert_eq!(link.to_object_role, String::from(""));
}



// +------------------------------------+
// |        Test OBJ Object Model       |
// +------------------------------------+

#[test]
fn test_obj_model(){
    let object_model = match obj_object_model(&b"Object:lg\nlieblingsgrieche:Restaurant\nkategorie:Sterne 3\nname \"Platon\"\n\nObject:maren\nmaren:Gast\nstatus \"Koenig\"\ngeldbetrag:EUR 300\n\nLink:besucht\nmaren,lg\n\n/Model "[..]){
        Ok(val) => {
            println!("Found link: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    let objects = object_model.objects;
    for object in &objects {
        println!("Object: {:?}", object);
    }

    let links = object_model.links;
    for link in &links {
        println!("Link: {:?}", link);
    }

//    assert!(false);
}





// +------------------------------------+
// |        Test PACK Package name      |
// +------------------------------------+

#[test]
fn test_pack_package_name(){
    assert_eq!(pack_package_name(&b" Package:AVeryLongName\n"[..]), Ok((&b"\n"[..], String::from("AVeryLongName"))));
}

// +------------------------------------+
// |      Test PACK Sub Packages        |
// +------------------------------------+

#[test]
fn test_pack_sub_packages(){
    assert_eq!(pack_sub_packages(&b"\nSub1,Sub2,Sub3,AVeryLongName\n"[..]), Ok((&b"\n"[..], vec![String::from("Sub1"), String::from("Sub2"), String::from("Sub3"), String::from("AVeryLongName")])));
}


// +------------------------------------+
// |      Test PACK Package             |
// +------------------------------------+

#[test]
fn test_pack_package(){
    let package = match pack_package(&b" Package:Main\nSub1,Sub2,Sub3\n\n "[..]){
        Ok(val) => {
            println!("Found link: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(package.package_name, String::from("Main"));
    assert_eq!(package.inner_packages.is_some(), true);
    assert_eq!(package.inner_packages.unwrap(), vec![String::from("Sub1"), String::from("Sub2"), String::from("Sub3")]);
}

#[test]
fn test_pack_package2(){
    let package = match pack_package(&b" Package:Main\n\n "[..]){
        Ok(val) => {
            println!("Found link: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(package.package_name, String::from("Main"));
    assert_eq!(package.inner_packages.is_some(), false);
}


// +------------------------------------+
// |        Test PACK Relation          |
// +------------------------------------+

#[test]
fn test_pack_relation(){
    let relation = match pack_relation(&b"Import\nMain,Lib1\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(relation.package_rel_name, PackageRelName::Import);
    assert_eq!(relation.from_package, String::from("Main"));
    assert_eq!(relation.to_package, String::from("Lib1"));
}



// +------------------------------------+
// |      Test PACK Package Model       |
// +------------------------------------+

#[test]
fn test_pack_package_model(){
    let package_model = match pack_package_model(&b"\nPackage:Main\nSub1,Sub2,Sub3\n\nPackage:Sub1\nSubSub1,SubSub2\n\nPackage:Sub2\n\nPackage:Sub3\n\nPackage:SubSub1\n\nPackage:SubSub2\n\nPackage:Other1\n\nPackage:Other2\n\nImport\nMain,Other1\n\nImport\nMain,Other2\n\n/Model "[..]){
        Ok(val) => {
            println!("Found package model: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Package model parsing unsuccessful: {}", err);
            return;
        }
    };

    let packages = package_model.packages;
    for package in &packages {
        println!("Package: {:?}", package);
    }

    let relations = package_model.relations;
    for relation in &relations {
        println!("Relation: {:?}", relation);
    }

//    assert!(false);
}



// +------------------------------------+
// |           Test UC Actor            |
// +------------------------------------+

#[test]
fn test_uc_actor(){
    let actor = match uc_actor(&b"Actor:User\n\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(actor.name_intern, String::from("User"));
    assert_eq!(actor.name_display, String::from("User"));
}

#[test]
fn test_uc_actor2(){
    let actor = match uc_actor(&b"Actor:u\nUser\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(actor.name_intern, String::from("u"));
    assert_eq!(actor.name_display, String::from("User"));
}



// +------------------------------------+
// |         Test UC AARelation         |
// +------------------------------------+

#[test]
fn test_uc_actor_actor_relation(){
    let aa_relation = match uc_actor_actor_relation(&b"Generalization\nm,User\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(aa_relation.from_actor, String::from("m"));
    assert_eq!(aa_relation.to_actor, String::from("User"));
    assert_eq!(aa_relation.border_type, BorderType::Solid);
//  assert_eq!(aa_relation.arrow_type, RelationArrow::TriangleFilled); //Meaningless currently
}


// +------------------------------------+
// |        Test UC UCUCRelation        |
// +------------------------------------+

#[test]
fn test_uc_use_case_use_case_relation(){
    let ucuc_relation = match uc_use_case_use_case_relation(&b"Include\nverifyLogin,login\n\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(ucuc_relation.from_use_case, String::from("verifyLogin"));
    assert_eq!(ucuc_relation.to_use_case, String::from("login"));
    assert_eq!(ucuc_relation.border_arrow_type, (BorderType::Dashed, RelationArrow::Arrow));
    assert_eq!(ucuc_relation.note, None);
    assert_eq!(ucuc_relation.relation_type, UseCaseRelationType::Include);
}

#[test]
fn test_uc_use_case_use_case_relation2(){
    let ucuc_relation = match uc_use_case_use_case_relation(&b"Extend\nverifyLogin,login\nDies ist eine Notiz\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(ucuc_relation.from_use_case, String::from("verifyLogin"));
    assert_eq!(ucuc_relation.to_use_case, String::from("login"));
    assert_eq!(ucuc_relation.border_arrow_type, (BorderType::Dashed, RelationArrow::Arrow));
    assert_eq!(ucuc_relation.note, Some(String::from("Dies ist eine Notiz")));
    assert_eq!(ucuc_relation.relation_type, UseCaseRelationType::Extend);
}


// +------------------------------------+
// |        Test UC AUCRelation        |
// +------------------------------------+

#[test]
fn test_uc_actor_use_case_relation(){
    let auc_relation = match uc_actor_use_case_relation(&b"Association\nUser,login\n"[..]){
        Ok(val) => {
            println!("Found relation: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Relation parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(auc_relation.from_actor, String::from("User"));
    assert_eq!(auc_relation.to_use_case, String::from("login"));
    assert_eq!(auc_relation.border_type, BorderType::Solid);
    assert_eq!(auc_relation.arrow_type, RelationArrow::None);
}



// +------------------------------------+
// |           Test UC UseCase          |
// +------------------------------------+

#[test]
fn test_uc_use_case(){
    let use_case = match uc_use_case(&b"UseCase:Anmeldedaten verifizieren\n\n"[..]){
        Ok(val) => {
            println!("Found Use Case: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Use Case parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(use_case.name_intern, String::from("Anmeldedaten verifizieren"));
    assert_eq!(use_case.name_display, String::from("Anmeldedaten verifizieren"));
}

#[test]
fn test_uc_use_case2(){
    let use_case = match uc_use_case(&b"UseCase:verifyLogin\nAnmeldedaten verifizieren\n"[..]){
        Ok(val) => {
            println!("Found Use Case: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Use Case parsing unsuccessful: {}", err);
            return;
        }
    };

    assert_eq!(use_case.name_intern, String::from("verifyLogin"));
    assert_eq!(use_case.name_display, String::from("Anmeldedaten verifizieren"));
}


// +------------------------------------+
// |      Test PACK Package Model       |
// +------------------------------------+

#[test]
fn test_uc_use_case_model(){
    let use_case_model = match uc_use_case_model(&b"\nSystem:Web Shop\n\nActor:User\n\nActor:m\nMember\n\nActor:adm\nAdmin\n\nGeneralization\nm,User\n\nGeneralization\nadm,m\n\nUseCase:login\nEinloggen\n\nUseCase:verifyLogin\nAnmeldedaten verifizieren\n\nGeneralize\nlogin,verifyLogin\n\nInclude\nverifyLogin,login\nIrgendeine Notiz die lang ist\n\nExtend\ngamble,winMoney\nWenn Spiel gewonnen\n\nAssociation\nUser,login\n/Model "[..]){
        Ok(val) => {
            println!("Found package model: {:?}", val.1);
            val.1
        },
        Err(err) => {
            assert!(false, "Package model parsing unsuccessful: {}", err);
            return;
        }
    };

    println!("System Name: {}", use_case_model.system.name);

    let use_cases = use_case_model.use_cases;
    for use_case in &use_cases {
        println!("UseCase: {:?}", use_case);
    }

    let actors = use_case_model.actors;
    for actor in &actors {
        println!("Actor: {:?}", actor);
    }

    let actor_actor_relations = use_case_model.actor_actor_relations;
    for actor_actor_relation in &actor_actor_relations {
        println!("AA_Relation: {:?}", actor_actor_relation);
    }

    let actor_use_case_relations = use_case_model.actor_use_case_relations;
    for actor_use_case_relation in &actor_use_case_relations {
        println!("AUC_Relation: {:?}", actor_use_case_relation);
    }

    let use_case_use_case_relations = use_case_model.use_case_use_case_relations;
    for use_case_use_case_relation in &use_case_use_case_relations {
        println!("UCUC_Relation: {:?}", use_case_use_case_relation);
    }

//    assert!(false);
}

/*
#[derive(Debug)]
pub struct UseCaseModel{
    pub system: System,
    pub use_cases: Vec<UseCase>,
    pub actors: Vec<Actor>,
    pub actor_actor_relations: Vec<ActorActorRelation>,
    pub actor_use_case_relations: Vec<ActorUseCaseRelation>,
    pub use_case_use_case_relations: Vec<UseCaseUseCaseRelation>
}

named!(uc_use_case_model<&[u8], UseCaseModel>,
    do_parse!(
        system: uc_system >>
        actors: many1!(uc_actor) >>
        aa_relation: many0!(uc_actor_actor_relation) >>
        use_cases: many1!(uc_use_case) >>
        ucuc_relation: many0!(uc_use_case_use_case_relation) >>uc_actor_use_case_relation
        auc_relation: many_till!(uc_actor_use_case_relation, pair!(take_while!(is_ws), tag!(&b"/Model"[..]))) >>
        (UseCaseModel { system, use_cases, actors, actor_actor_relations: aa_relation, use_case_use_case_relation: ucuc_relation, actor_use_case_relation: auc_relation })
    )
);
*/