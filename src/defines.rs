use std::collections::HashMap;
use rusttype::{point, Font, Scale};
use std::fmt;
use std::error::Error as StdError;

//========== Structs and enums for layouting and drawing ==========
#[derive(Debug)]
pub struct BoxLayout {
    pub lt: XY,
    pub rt: XY,
    pub lb: XY,
    pub rb: XY,
    pub height: u32,
    pub width: u32,
    pub uneven: bool
}

#[derive(Debug)]
pub struct LineLayout {
    pub start: XY,
    pub end: XY,
    pub base_first: u32,
    pub gap_first: f32,
    pub gap_second: f32
}

#[derive(Debug)]
pub struct Colors {
    pub white: image::Bgra<u8>,
    pub black: image::Bgra<u8>,
}

#[derive(Debug)]
pub struct Scales {
    pub one: Scale,
    pub two: Scale,
}

#[derive(Debug)]
pub struct XY {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct General {
    pub imgxy: XY,
    pub colors: Colors,
    pub scales : Scales,
}
//========================================

//======== Error types for parsing =======

#[derive(Debug)]
pub enum ParseError {
    InvalidModelError,
    ParseError,
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidModelError => f.write_str("InvalidModelError"),
            ParseError::ParseError => f.write_str("ParseError"),
        }
    }
}
impl StdError for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidModelError => "The provided model is not valid!",
            ParseError::ParseError => "The syntax is wrong",
        }
    }
}



//========================================

//========== Structs and enums for modelling ==========
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ClassType{
    SimpleClass,
    AbstractClass,
    ActiveClass,
    VarBorderClass,
    DashedBorderClass,
    None
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RelationType{
    Association,
    Inheritance,
    Implementation,
    Dependency,
    Aggregation,
    Composition,
    None
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RelationArrow{
    Arrow,
    TriangleEmpty,
//  TriangleFilled,
    DiamondEmpty,
    DiamondFilled,
    None
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BorderType{
    Solid,
    Dashed,
    None
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TextDecoration{
    Bold,
    Italic,
    BoldItalic,
    Underlined,
    HorizontalLine,
    None
}

#[derive(Debug)]
pub struct Line{
    pub content: String,
    pub decor: TextDecoration
}

#[derive(Debug)]
pub struct Class{
    pub class_type: ClassType,
    pub class_name: String,
    pub class_stereotype: String,
    pub border_width: i32,
    pub lines: Vec<Line>
}

#[derive(Debug)]
pub struct Relation{
    pub border_type: BorderType,
    pub arrow_type: RelationArrow,
    pub from_class: String,
    pub from_class_card: String,
    pub to_class: String,
    pub to_class_card: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Visibility{
    Public,
    Package,
    Protected,
    Private,
    None
}

#[derive(Debug)]
pub struct ClassModel{
    pub classes: Vec<Class>,
    pub relations: Vec<Relation>
}



#[derive(Debug)]
pub struct Object{
    pub object_title: String,
    pub object_intern_name: String,
    pub content_lines: Vec<String>
}

#[derive(Debug)]
pub struct Link{
    pub link_name: String,
    pub from_object: String,
    pub from_object_role: String,
    pub to_object: String,
    pub to_object_role: String
}

#[derive(Debug)]
pub struct ObjectModel{
    pub objects: Vec<Object>,
    pub links: Vec<Link>
}



#[derive(Debug)]
pub struct Package{
    pub package_name: String,
    pub inner_packages: Option<Vec<String>>
}

#[derive(Debug)]
pub struct PackageRelation{
    pub package_rel_name: PackageRelName,
    pub from_package: String,
    pub to_package: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PackageRelName{
    Import,
    Access,
    Merge,
    None
}

#[derive(Debug)]
pub struct PackageModel{
    pub packages: Vec<Package>,
    pub relations: Vec<PackageRelation>
}



#[derive(Debug)]
pub struct System{
    pub name: String
}

#[derive(Debug)]
pub struct UseCase{
    pub name_intern: String,
    pub name_display: String
}

#[derive(Debug)]
pub struct Actor{
    pub name_intern: String,
    pub name_display: String
}

#[derive(Debug)]
pub struct ActorUseCaseRelation {
    pub border_type: BorderType,
    pub arrow_type: RelationArrow,
    pub from_actor: String,
    pub to_use_case: String
}

#[derive(Debug)]
pub struct ActorActorRelation {
    pub border_type: BorderType,
    pub arrow_type: RelationArrow,
    pub from_actor: String,
    pub to_actor: String
}

#[derive(Debug)]
pub struct UseCaseUseCaseRelation {
    pub relation_type: UseCaseRelationType,
    pub border_arrow_type: (BorderType, RelationArrow),
    pub from_use_case: String,
    pub to_use_case: String,
    pub note: Option<String>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum UseCaseRelationType{
    Include,
    Extend,
    Generalize
}

#[derive(Debug)]
pub struct UseCaseModel{
    pub system: System,
    pub use_cases: Vec<UseCase>,
    pub actors: Vec<Actor>,
    pub actor_actor_relations: Vec<ActorActorRelation>,
    pub actor_use_case_relations: Vec<ActorUseCaseRelation>,
    pub use_case_use_case_relations: Vec<UseCaseUseCaseRelation>
}

#[derive(Debug)]
pub struct ModelContainer{
    pub model_type: ModelType,
    pub class_model: Option<ClassModel>,
    pub object_model: Option<ObjectModel>,
    pub package_model: Option<PackageModel>,
    pub use_case_model: Option<UseCaseModel>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ModelType{
    ClassModel,
    ObjectModel,
    PackageModel,
    UseCaseModel,
    None
}

trait Modeltype {
    fn get_type(&self) -> String;
}

impl Modeltype for ClassModel {
    fn get_type(&self) -> String {
        "class".to_string()
    }
}

impl Modeltype for ObjectModel {
    fn get_type(&self) -> String {
        "object".to_string()
    }
}

impl Modeltype for PackageModel {
    fn get_type(&self) -> String {
        "package".to_string()
    }
}

impl Modeltype for UseCaseModel {
    fn get_type(&self) -> String {
        "usecase".to_string()
    }
}
//========================================