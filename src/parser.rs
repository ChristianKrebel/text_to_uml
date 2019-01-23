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
        take_while!(is_ws) >>
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

named!(cd_method_pair<&[u8], String>,
    do_parse!(
        take_while!(is_ws) >>
        data_type: parse_till_ws >>
        var_name: parse_till_newline >>
        (format!("{}: {}", var_name, data_type))
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
        ( Line{ content: format!("{} {}", vis, variable), decor } )
    )
);

named!(cd_line<&[u8], Line>,
    do_parse!(
        line: alt_complete!(
            cd_horizontal_line |
            cd_member          |
            cd_method
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
//        relations: many0!(cd_relation) >>
        relations: many_till!(cd_relation, pair!(take_while!(is_ws), tag!(&b"/Model"[..]))) >>
//        tag!(&b"/Model"[..]) >>
        (ClassModel { classes, relations: relations.0 })
    )
);


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

        count = count + count;
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
            return Err(ParseError::InvalidModelError);
        },
        ModelType::PackageModel => {
            return Err(ParseError::InvalidModelError);
        },
        ModelType::UseCaseModel => {
            return Err(ParseError::InvalidModelError);
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

// ===============================





/*
 * Test code
 */

#[test]
fn test_test1(){
    assert_eq!(test(&b"abababab  z"[..]), Ok((&b""[..], vec![String::from("ab"), String::from("ab"), String::from("ab"), String::from("ab")])));
}

// +------------------------------------+
// |          Test model type           |
// +------------------------------------+



#[test]
fn test_model_type1(){
    assert_eq!(model_type("Model:ClassDiagram"), Ok(("", ModelType::ClassModel)));
}

#[test]
fn test_model_type2(){
    assert_eq!(model_type("        Model:   ObjectDiagram"), Ok(("", ModelType::ObjectModel)));
}

#[test]
fn test_model_type3(){
    assert_eq!(model_type(" Model: PackageDiagram"), Ok(("", ModelType::PackageModel)));
}

#[test]
fn test_model_type4(){
    assert_eq!(model_type("Model: UseCaseDiagram"), Ok(("", ModelType::UseCaseModel)));
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


#[test]
fn test_parse_till_gt(){
    assert_eq!(parse_till_gt(&b"  sumWeirdStr<>sdsd "[..]), Ok((&b">sdsd "[..], "sumWeirdStr<")));
}



//#[test]
//fn test_cd_horizontal_line(){
//    assert_eq!(cd_horizontal_line(&b"  -- "[..]), Ok((&b" "[..], (String::from(""), TextDecoration::HorizontalLine))));
//}



#[test]
fn test_cd_variable_pair(){
    assert_eq!(cd_variable_pair(&b"  boolean someVarName "[..]), Ok((&b" "[..], String::from("someVarName: boolean"))));
}

// +------------------------------------+
// |   Test class diagram member line   |
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



/*
 Complete class test
*/

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


/*
 Complete Relation test
*/

#[test]
fn test_cd_relation_complete1(){
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