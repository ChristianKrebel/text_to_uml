use std::collections::HashMap;

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
pub struct Class{
    pub class_type: ClassType,
    pub class_name: String,
    pub class_stereotype: String,
    pub border_width: i32,
    pub content_lines: Vec<String>,
    pub content_decor: Vec<TextDecoration>
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
    pub object_name: String,
    pub belonging_class: String,
    pub content_lines: Vec<String>
}

#[derive(Debug)]
pub struct Link{
    pub link_name: String,
    pub from_object: String,
    pub from_class_role: String,
    pub to_object: String,
    pub to_class_role: String
}

#[derive(Debug)]
pub struct ObjectModel{
    pub objects: Vec<Object>,
    pub links: Vec<Link>
}



#[derive(Debug)]
pub struct Package{
    pub package_name: String,
    pub inner_packages: Vec<Package>
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
    Acces,
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
    pub system_name: String
}

#[derive(Debug)]
pub struct UseCase{
    pub content_lines: Vec<String>
}

#[derive(Debug)]
pub struct Participant{
    pub participant_name: String
}

#[derive(Debug)]
pub struct UseCaseRelation{
    pub participant_rel_name: ParticipantRelName,
    pub border_type: BorderType,
    pub arrow_type: RelationArrow,
    pub from: String,
    pub to: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParticipantRelName{
    Include,
    Extend,
    None
}

#[derive(Debug)]
pub struct UseCaseModel{
    pub system: System,
    pub use_cases: Vec<UseCase>,
    pub participants: Vec<Participant>,
    pub relations: Vec<UseCaseRelation>
}