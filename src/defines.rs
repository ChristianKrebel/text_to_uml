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
    pub border_width: i32,
    pub content_lines: Vec<String>,
    pub content_decor: Vec<TextDecoration>
}

#[derive(Debug)]
pub struct Relation{
    border_type: BorderType,
    from_class: String,
    from_arrow_head: RelationType,
    to_class: String,
    to_arrow_head: RelationType
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