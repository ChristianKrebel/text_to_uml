#[allow(unused_variables, unused_mut, unused)]

extern crate image;
extern crate imageproc;
extern crate rand;


use defines::*;

use self::imageproc::rect::*;
use self::imageproc::drawing::*;
use rusttype::{point, Font, Scale};

use std::vec::Vec;
use std::fs::File;
use std::path::Path;
use std::ptr::null;
use std::str::*;
use std::string::*;
use std::mem;
use std::num::Wrapping;
use self::rand::Rng;

use self::image::{DynamicImage, GenericImage, Pixel, Rgba, RgbaImage, ImageFormat};

pub struct ClassLayout {
    lt: XY,
    rt: XY,
    lb: XY,
    rb: XY,
    height: u32,
    width: u32,
    uneven: bool
}

struct Colors {
    white: image::Rgba<u8>,
    black: image::Rgba<u8>,
    red: image::Rgba<u8>,
    blue: image::Rgba<u8>
}
struct Scales {
    one: Scale,
    two: Scale,
}
pub struct XY {
    x: u32,
    y: u32,
}
pub struct General {
    buffer: image::RgbaImage,
    imgxy: XY,
    colors: Colors,
    scales : Scales,
}

const LINE_HEIGHT: u32 = 30;
const LETTER_WIDTH: u32 = 16;
const PADDING_LEFT: u32 = 8;
const PADDING_TOP: u32 = 2;
const RELATION_STICK: u32 = 50;
const DASHED_LENGTH: u32 = 5;
const DASHED_LENGTH2: u32 = DASHED_LENGTH * 5;
const REL_GAP_DISTANCE: f32 = 25.0;
const ARROW_SIZE: u32 = 20;
const ACTIVE_PADDING: u32 = PADDING_LEFT * 2;
const CARD_DIST: u32 = 4;

pub fn generate_pic(class_vec: &mut Vec<Class>, rel_vec: &mut Vec<Relation>) {
    let path = Path::new("output.png");

    // ------ Layouting all classes ------
    let mut class_layout_vec: Vec<ClassLayout> = Vec::new();
    let mut class_count = class_vec.len();

    // calc distance between upper and lower classes
    let RELATION_GAP: u32 = ((RELATION_STICK * 2) as f32 + (rel_vec.len() as f32 + 1.0) * REL_GAP_DISTANCE) as u32;

    // calc heights for upper half of classes (uneven)
    let mut greatest_height_first_half: u32 = 0;
    for (i,c) in class_vec.iter().enumerate() {
        let mut greatest_height: u32 = 0;
        if i % 2 != 0 {
            if !c.class_name.is_empty() {
                greatest_height += 1;
            }
            if !c.class_stereotype.is_empty() {
                greatest_height += 1;
            }
            greatest_height += c.content_lines.len() as u32;
        }
        if greatest_height > greatest_height_first_half {
            greatest_height_first_half = greatest_height;
        }
    }

    // calc heights for lower half of classes (even)
    let mut greatest_height_second_half: u32 = 0;
    for (i,c) in class_vec.iter().enumerate() {
        let mut greatest_height: u32 = 0;
        if i % 2 == 0 {
            if !c.class_name.is_empty() {
                greatest_height += 1;
            }
            if !c.class_stereotype.is_empty() {
                greatest_height += 1;
            }
            greatest_height += c.content_lines.len() as u32;
        }
        if greatest_height > greatest_height_second_half {
            greatest_height_second_half = greatest_height;
        }
    }

    greatest_height_first_half *= LINE_HEIGHT;
    greatest_height_second_half *= LINE_HEIGHT;
    let mut base_line_first_half: u32 = greatest_height_first_half + 50;
    let mut top_line_second_half: u32 = if class_count == 1
        {base_line_first_half} else {base_line_first_half + RELATION_GAP};


    let mut last_left_distance_uneven: u32 = 50;
    let mut last_left_distance_even: u32 = 50;

    for (i,c) in class_vec.iter().enumerate() {

        let mut greatest_width: u32 = 0;
        for line in c.content_lines.iter() {
            if line.len() as u32 > greatest_width {
                greatest_width = line.len() as u32;
            }
        }
        if !c.class_name.is_empty() {
            if c.class_name.len() as u32 > greatest_width {
                greatest_width = c.class_name.len() as u32;
            }
        }
        if !c.class_stereotype.is_empty() {
            if c.class_stereotype.len() as u32 > greatest_width {
                greatest_width = c.class_stereotype.len() as u32;
            }
        }
        greatest_width *= LETTER_WIDTH;

        let mut height: u32 = 0;

            if !c.class_name.is_empty() {
                height += 1;
            }
            if !c.class_stereotype.is_empty() {
                height += 1;
            }
            height += c.content_lines.len() as u32;

        height *= LINE_HEIGHT;

        let mut lb: XY = XY {x: 0, y: 0};
        let mut rb: XY = XY {x: 0, y: 0};
        let mut lt: XY = XY {x: 0, y: 0};
        let mut rt: XY = XY {x: 0, y: 0};
        if i % 2 != 0 {
            lb = XY {x: last_left_distance_uneven, y: base_line_first_half};
            rb = XY {x: &lb.x + &greatest_width, y: lb.y};
            lt = XY {x: lb.x, y: &lb.y - &height};
            rt = XY {x: rb.x, y: lt.y};
        } else {
            lt = XY {x: last_left_distance_even, y: top_line_second_half};
            rt = XY {x: &lt.x + &greatest_width, y: lt.y};
            lb = XY {x: lt.x, y: &lt.y + &height};
            rb = XY {x: rt.x, y: lb.y};
        }

        let uneven: bool = if i % 2 != 0 {true} else {false};


        let class_layout: ClassLayout = ClassLayout {
            lt: lt,
            rt: rt,
            lb: lb,
            rb: rb,
            height: height,
            width: greatest_width,
            uneven: uneven,
        };
        class_layout_vec.push(class_layout);
        if i % 2 != 0 {
            last_left_distance_uneven += &greatest_width + 100;
        } else {
            last_left_distance_even += &greatest_width + 100;
        }
    }

    // ------------



    // Calc picture bounds
    let mut greatest_last_left_distance: u32 = if last_left_distance_uneven > last_left_distance_even
        {last_left_distance_uneven - 50} else {last_left_distance_even - 50};
    let xy: XY = XY {
        x: greatest_last_left_distance,
        y: top_line_second_half + greatest_height_second_half + 50,
    };

    // Colors
    let colors: Colors = Colors {
        white: Rgba([255u8, 255u8, 255u8, 255u8]),
        black: Rgba([0u8, 0u8, 0u8, 255u8]),
        red: Rgba([255u8, 0u8, 0u8, 127u8]),
        blue: Rgba([0u8, 0u8, 255u8, 127u8]),
    };

    // Fonts
    let mut font_vec: Vec<Font> = Vec::new();

    // Load the font
    let font_data = include_bytes!("../fonts/UbuntuMono-R.ttf");
    // This only succeeds if collection consists of one font
    font_vec.push(Font::from_bytes(font_data as &[u8]).expect("Error constructing Font"));
    // Load the font
    let font_data2 = include_bytes!("../fonts/UbuntuMono-RI.ttf");
    // This only succeeds if collection consists of one font
    font_vec.push(Font::from_bytes(font_data2 as &[u8]).expect("Error constructing Font"));
    // Load the font
    let font_data3 = include_bytes!("../fonts/UbuntuMono-B.ttf");
    // This only succeeds if collection consists of one font
    font_vec.push(Font::from_bytes(font_data3 as &[u8]).expect("Error constructing Font"));
    // Load the font
    let font_data4 = include_bytes!("../fonts/UbuntuMono-BI.ttf");
    // This only succeeds if collection consists of one font
    font_vec.push(Font::from_bytes(font_data4 as &[u8]).expect("Error constructing Font"));

    // The font size to use
    let scales: Scales = Scales {
        one: Scale::uniform(18.0),
        two: Scale::uniform(26.0),
    };


    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::RgbaImage::new(xy.x, xy.y);

    // Most important general info
    let general: General = General {
        buffer: image::RgbaImage::new(xy.x, xy.y),
        imgxy: xy,
        colors: colors,
        scales: scales,
    };

    // Draw background
    draw_filled_rect_mut(
        &mut imgbuf, imageproc::rect::Rect::at(0, 0).of_size(general.imgxy.x, general.imgxy.y),
        general.colors.white);


    // ------ DRAW ------
    for (i, c) in class_vec.iter().enumerate() {
        draw_class(&mut imgbuf, &general, &font_vec, &c, &class_layout_vec[i]);
    }

    // ------ Layouting all relations ------

    let mut rel_gap_first = REL_GAP_DISTANCE;
    let mut rel_gap_second = REL_GAP_DISTANCE;

    let mut all_to_class_rels_vec: Vec<Vec<bool>> = Vec::new();
    for (i, c) in class_vec.iter().enumerate() {
        let mut empty_vec: Vec<bool> = Vec::new();
        empty_vec.push(true);
        all_to_class_rels_vec.push(empty_vec);
    }


    for (i, c) in class_vec.iter().enumerate() {
        let mut to_class_rels_vec: Vec<bool> = Vec::new();
        // Durch alle Relationen
        for (index, rel) in rel_vec.iter().enumerate() {
            // Wenn Relation eingeht, dann speichere Index der Relation
            if rel.to_class == c.class_name {
                to_class_rels_vec.push(false);
            }
        }
        all_to_class_rels_vec[i] = to_class_rels_vec;
    }


    // Durch alle Klassen
    for (i,c) in class_vec.iter().enumerate() {
        let mut rel_starts: Vec<XY> = Vec::new();
        let mut rel_starts_stepsize: u32;
        let mut rel_ends_stepsize: u32;
        let mut rels_indexes: Vec<usize> = Vec::new();
        let mut rels_indexes2: Vec<usize> = Vec::new();

        // Durch alle Relationen
        for (index, rel) in rel_vec.iter().enumerate() {
            // Wenn Relation ausgeht, dann speichere Index der Relation
            if rel.from_class == c.class_name {
                rels_indexes.push(index);
            }
            // Wenn Relation eingeht, dann speichere Index der Relation
            if rel.to_class == c.class_name {
                rels_indexes2.push(index);
            }
        }


        rel_starts_stepsize = (class_layout_vec[i].width/2) / (rels_indexes.len() as u32 + 1);


        let mut x_start: u32 = 0;
        let mut y_start: u32 = 0;
        let mut x_end: u32 = 0;
        let mut y_end: u32 = 0;

        if class_layout_vec[i].uneven {
            x_start = class_layout_vec[i].lb.x;
            y_start = class_layout_vec[i].lb.y;
        } else {
            x_start = class_layout_vec[i].lt.x;
            y_start = class_layout_vec[i].lt.y;
        }



        // Durch alle Indexe der Relationen, die aus der Klasse gehen^
        for index in rels_indexes {

            // Durch alle Relationen
            for (l, rel) in rel_vec.iter().enumerate() {
                // Wenn Index der Relation der Klasse dem Index der durchlaufenden Relation ist
                if index == l {

                    x_start += rel_starts_stepsize;
                    let mut xy1: XY = XY {
                        x: x_start,
                        y: y_start
                    };

                    let mut to_class_i: usize = 0;
                    for (ci, c) in class_vec.iter().enumerate() {
                        if c.class_name == rel.to_class {
                            to_class_i = ci;
                        }
                    }
                    rel_ends_stepsize = (class_layout_vec[to_class_i].width/2) / (all_to_class_rels_vec[to_class_i].len() as u32 + 1);
                    if class_layout_vec[to_class_i].uneven {
                        x_end = class_layout_vec[to_class_i].lb.x + (class_layout_vec[to_class_i].width/2);
                        y_end = class_layout_vec[to_class_i].lb.y;
                    } else {
                        x_end = class_layout_vec[to_class_i].lt.x + (class_layout_vec[to_class_i].width/2);
                        y_end = class_layout_vec[to_class_i].lt.y;
                    }

                    let mut multip: u32 = 1;
                    for (i, vector) in all_to_class_rels_vec.iter_mut().enumerate() {
                        if i == to_class_i {
                            for l in 0..vector.len() {
                                if vector[l] == true {
                                    multip += 1;
                                } else {
                                    vector[l] = true;
                                    break;
                                }
                            }
                        }
                    }
                    x_end += rel_ends_stepsize * multip;
                    let mut xy2: XY = XY {
                        x: x_end,
                        y: y_end
                    };

                    let mut zwerg = draw_rel(&mut imgbuf, &general, &font_vec,
                                             &rel, &xy1, &xy2,
                                             base_line_first_half,
                                             rel_gap_first, rel_gap_second);
                    rel_gap_first = zwerg[0];
                    rel_gap_second = zwerg[1];
                }
            }
        }
    }


    // Save the picture
    imgbuf.save(&path).unwrap();
}

pub fn draw_class(buffer: &mut image::RgbaImage, general: &General, fonts: &Vec<Font>, class: &Class,
                  class_layout: &ClassLayout) {

    //let &buffer = &general.buffer;
    let x = general.imgxy.x;
    let y = general.imgxy.y;
    let colors = &general.colors;
    let scales = &general.scales;


    match class.class_type {
        ClassType::SimpleClass => {
            // Outer borderline
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(
                    class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width, class_layout.height),
                colors.black);

            let mut height_to_write_at: u32 = class_layout.lt.y + PADDING_TOP;
            let mut has_stereotype: bool = if class.class_stereotype.is_empty() { false } else { true };
            let mut is_abstract: bool = if class.class_stereotype == "<<abstract>>" { true } else { false };

            // Draw name (and stereotype)
            if class.class_type == ClassType::AbstractClass {
                has_stereotype = true;
                is_abstract = true;
            }
            if has_stereotype {
                if is_abstract {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[1], &class.class_name);
                    height_to_write_at += LINE_HEIGHT;
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_stereotype);
                    height_to_write_at += LINE_HEIGHT;
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_name);
                    height_to_write_at += LINE_HEIGHT;
                }
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                    height_to_write_at, scales.two, &fonts[0], &class.class_name);
                height_to_write_at += LINE_HEIGHT;
            }

            // Draw all other lines of text or just lines
            let mut deco_font: u32 = 0;
            for (i, line) in class.content_lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                let mut is_underlined: bool = false;
                match class.content_decor[i] {
                    TextDecoration::None => {
                    }
                    TextDecoration::HorizontalLine => {
                        is_horizontal_line = true;
                    }
                    TextDecoration::Bold => {
                        deco_font = 2;
                    }
                    TextDecoration::Italic => {
                        deco_font = 1;
                    }
                    TextDecoration::BoldItalic => {
                        deco_font = 3;
                    }
                    TextDecoration::Underlined => {
                        is_underlined = true;
                    }
                }
                if is_horizontal_line || line.is_empty() || line == "-" {
                    draw_hollow_rect_mut(
                        buffer, imageproc::rect::Rect::at(
                            class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                            class_layout.width,
                            height_to_write_at - class_layout.lt.y + (LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + PADDING_LEFT) as f32,
                                               height_to_write_at as f32 + LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + PADDING_LEFT) as f32 +
                                                   (LETTER_WIDTH as f32 * (line.len() as f32 - 1.0)),
                                               height_to_write_at as f32 + LINE_HEIGHT as f32 - 6.0),
                                              general.colors.black);
                    }
                }
                height_to_write_at += LINE_HEIGHT;
            }
        }
        ClassType::AbstractClass => {
            // Outer borderline
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(
                    class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width, class_layout.height),
                colors.black);

            let mut height_to_write_at: u32 = class_layout.lt.y + PADDING_TOP;
            let mut has_stereotype: bool = if class.class_stereotype.is_empty() { false } else { true };
            let mut is_abstract: bool = if class.class_stereotype == "<<abstract>>" { true } else { false };

            // Draw name (and stereotype)
            if class.class_type == ClassType::AbstractClass {
                has_stereotype = true;
                is_abstract = true;
            }
            if has_stereotype {
                if is_abstract {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[1], &class.class_name);
                    height_to_write_at += LINE_HEIGHT;
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_stereotype);
                    height_to_write_at += LINE_HEIGHT;
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_name);
                    height_to_write_at += LINE_HEIGHT;
                }
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                    height_to_write_at, scales.two, &fonts[0], &class.class_name);
                height_to_write_at += LINE_HEIGHT;
            }

            // Draw all other lines of text or just lines
            let mut deco_font: u32 = 0;
            for (i, line) in class.content_lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                let mut is_underlined: bool = false;
                match class.content_decor[i] {
                    TextDecoration::None => {
                    }
                    TextDecoration::HorizontalLine => {
                        is_horizontal_line = true;
                    }
                    TextDecoration::Bold => {
                        deco_font = 2;
                    }
                    TextDecoration::Italic => {
                        deco_font = 1;
                    }
                    TextDecoration::BoldItalic => {
                        deco_font = 3;
                    }
                    TextDecoration::Underlined => {
                        is_underlined = true;
                    }
                }
                if is_horizontal_line || line.is_empty() || line == "-" {
                    draw_hollow_rect_mut(
                        buffer, imageproc::rect::Rect::at(
                            class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                            class_layout.width,
                            height_to_write_at - class_layout.lt.y + (LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + PADDING_LEFT) as f32,
                                               height_to_write_at as f32 + LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + PADDING_LEFT) as f32 +
                                                   (LETTER_WIDTH as f32 * (line.len() as f32 - 1.0)),
                                               height_to_write_at as f32 + LINE_HEIGHT as f32 - 6.0),
                                              general.colors.black);
                    }
                }
                height_to_write_at += LINE_HEIGHT;
            }
        }
        ClassType::ActiveClass => {
            // Outer borderline
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(
                    class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width, class_layout.height),
                colors.black);
            // Inner borderline
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(
                    class_layout.lt.x as i32 + PADDING_LEFT as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width - ACTIVE_PADDING, class_layout.height),
                colors.black);

            let mut height_to_write_at: u32 = class_layout.lt.y + PADDING_TOP;
            let mut has_stereotype: bool = if class.class_stereotype.is_empty() { false } else { true };
            let mut is_abstract: bool = if class.class_stereotype == "<<abstract>>" { true } else { false };

            // Draw name (and stereotype)
            if class.class_type == ClassType::AbstractClass {
                has_stereotype = true;
                is_abstract = true;
            }
            if has_stereotype {
                if is_abstract {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[1], &class.class_name);
                    height_to_write_at += LINE_HEIGHT;
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[0], &class.class_stereotype);
                    height_to_write_at += LINE_HEIGHT;
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[0], &class.class_name);
                    height_to_write_at += LINE_HEIGHT;
                }
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + ACTIVE_PADDING,
                    height_to_write_at, scales.two, &fonts[0], &class.class_name);
                height_to_write_at += LINE_HEIGHT;
            }

            // Draw all other lines of text or just lines
            let mut deco_font: u32 = 0;
            for (i, line) in class.content_lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                let mut is_underlined: bool = false;
                match class.content_decor[i] {
                    TextDecoration::None => {
                    }
                    TextDecoration::HorizontalLine => {
                        is_horizontal_line = true;
                    }
                    TextDecoration::Bold => {
                        deco_font = 2;
                    }
                    TextDecoration::Italic => {
                        deco_font = 1;
                    }
                    TextDecoration::BoldItalic => {
                        deco_font = 3;
                    }
                    TextDecoration::Underlined => {
                        is_underlined = true;
                    }
                }
                if is_horizontal_line || line.is_empty() || line == "-" {
                    draw_hollow_rect_mut(
                        buffer, imageproc::rect::Rect::at(
                            class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                            class_layout.width,
                            height_to_write_at - class_layout.lt.y + (LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ACTIVE_PADDING) as f32,
                                               height_to_write_at as f32 + LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ACTIVE_PADDING) as f32 +
                                                   (LETTER_WIDTH as f32 * (line.len() as f32 - 1.0)),
                                               height_to_write_at as f32 + LINE_HEIGHT as f32 - 6.0),
                                              general.colors.black);
                    }
                }
                height_to_write_at += LINE_HEIGHT;
            }
        }
        ClassType::DashedBorderClass => {
        }
        ClassType::VarBorderClass => {
        }
        ClassType::None => {
        }
    }
}

pub fn draw_rel(buffer: &mut image::RgbaImage, general: &General, fonts: &Vec<Font>, rel: &Relation,
                start: &XY, end: &XY, base_first: u32, rel_gap_first: f32, rel_gap_second: f32) -> Vec<f32> {

    println!("from: {}, from card: {}", rel.from_class, rel.from_class_card);
    println!("to: {}, to card: {}", rel.to_class, rel.to_class_card);

    let mut is_in_first: bool = if start.y == base_first { true } else { false };
    let mut start_rel_y: f32 = if is_in_first {(start.y + RELATION_STICK) as f32} else {(start.y - RELATION_STICK) as f32};
    let mut rel_gap_first = rel_gap_first;
    let mut rel_gap_second = rel_gap_second;

    // Arrows
    match rel.arrow_type {
        RelationArrow::Arrow => {
            if end.y == base_first {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ARROW_SIZE as f32, end.y as f32 + ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ARROW_SIZE as f32, end.y as f32 + ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
            } else {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ARROW_SIZE as f32, end.y as f32 - ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ARROW_SIZE as f32, end.y as f32 - ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
            }

        }
        RelationArrow::TriangleEmpty => {
            if end.y == base_first {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ARROW_SIZE as f32, end.y as f32 + ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ARROW_SIZE as f32, end.y as f32 + ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ARROW_SIZE as f32, end.y as f32 + ARROW_SIZE as f32),
                                      (end.x as f32 - ARROW_SIZE as f32, end.y as f32 + ARROW_SIZE as f32),
                                      general.colors.black);
            } else {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ARROW_SIZE as f32, end.y as f32 - ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ARROW_SIZE as f32, end.y as f32 - ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ARROW_SIZE as f32, end.y as f32 - ARROW_SIZE as f32),
                                      (end.x as f32 - ARROW_SIZE as f32, end.y as f32 - ARROW_SIZE as f32),
                                      general.colors.black);
            }
        }
        RelationArrow::DiamondEmpty => {
            if is_in_first {
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ARROW_SIZE as f32, start.y as f32 + ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ARROW_SIZE as f32, start.y as f32 + ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ARROW_SIZE as f32, start.y as f32 + ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32 + (ARROW_SIZE*2) as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ARROW_SIZE as f32, start.y as f32 + ARROW_SIZE as f32),
                                      (start.x as f32, (start.y + ARROW_SIZE * 2) as f32),
                                      general.colors.black);
            } else {
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ARROW_SIZE as f32, start.y as f32 - ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ARROW_SIZE as f32, start.y as f32 - ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ARROW_SIZE as f32, start.y as f32 - ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32 - (ARROW_SIZE*2) as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ARROW_SIZE as f32, start.y as f32 - ARROW_SIZE as f32),
                                      (start.x as f32, (start.y - ARROW_SIZE * 2) as f32),
                                      general.colors.black);
            }
        }
        RelationArrow::DiamondFilled => {
            if is_in_first {
                let mut p1: Point<i32> = Point::new(start.x as i32, start.y as i32);
                let mut p2: Point<i32> = Point::new(start.x as i32 - ARROW_SIZE as i32, start.y as i32 + ARROW_SIZE as i32);
                let mut p3: Point<i32> = Point::new(start.x as i32, start.y as i32 + (ARROW_SIZE * 2) as i32);
                let mut p4: Point<i32> = Point::new(start.x as i32 + ARROW_SIZE as i32, start.y as i32 + ARROW_SIZE as i32);
                draw_convex_polygon_mut(buffer, &[p1, p2, p3, p4], general.colors.black);
            } else {
                let mut p1: Point<i32> = Point::new(start.x as i32, start.y as i32);
                let mut p2: Point<i32> = Point::new(start.x as i32 - ARROW_SIZE as i32, start.y as i32 - ARROW_SIZE as i32);
                let mut p3: Point<i32> = Point::new(start.x as i32, start.y as i32 - (ARROW_SIZE * 2) as i32);
                let mut p4: Point<i32> = Point::new(start.x as i32 + ARROW_SIZE as i32, start.y as i32 - ARROW_SIZE as i32);
                draw_convex_polygon_mut(buffer, &[p1, p2, p3, p4], general.colors.black);
            }
        }
        RelationArrow::None => {
            // Done
        }
    }

    // Lines
    match rel.border_type {
        BorderType::Solid => {
            let mut starty = start.y;
            let mut endy = end.y;
            // Little line / stick
            if rel.arrow_type == RelationArrow::DiamondEmpty {
                if is_in_first {
                    starty += (ARROW_SIZE*2);
                } else {
                    starty -= (ARROW_SIZE*2);
                }
            };
            if rel.arrow_type == RelationArrow::TriangleEmpty {
                if end.y == base_first {
                    endy += ARROW_SIZE;
                } else {
                    endy -= ARROW_SIZE;
                }
            };
            draw_line_segment_mut(buffer,
                                  (start.x as f32, starty as f32),
                                  (start.x as f32, start_rel_y as f32 + (
                                      if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                  general.colors.black);
            // Card. / multiplicities
            if !rel.from_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, start.x as u32 + CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.from_class_card);
            }
            if !rel.to_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, end.x as u32 + CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.to_class_card);
            }
            // Big lines
            draw_line_segment_mut(buffer,
                                  (start.x as f32, start_rel_y as f32 + (
                                      if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                  (end.x as f32, start_rel_y as f32 + (
                                      if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                  general.colors.black);
            draw_line_segment_mut(buffer,
                                  (end.x as f32, start_rel_y as f32 + (
                                      if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                  (end.x as f32, endy as f32),
                                  general.colors.black);
            if is_in_first {
                rel_gap_first += REL_GAP_DISTANCE;
            } else {
                rel_gap_second += REL_GAP_DISTANCE;
            }
        }
        BorderType::Dashed => {
            let mut endy = end.y;
            if rel.arrow_type == RelationArrow::TriangleEmpty {
                if end.y == base_first {
                    endy += ARROW_SIZE;
                } else {
                    endy -= ARROW_SIZE;
                }
            };

            let mut start_y_temp = start.y as f32;
            let mut start_x_temp = start.x as f32;
            // Card. / multiplicities
            if !rel.from_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, start.x as u32 + CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.from_class_card);
            }
            if !rel.to_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, end.x as u32 + CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.to_class_card);
            }
            // Little line / stick (FIRST)
            if is_in_first {
                println!("start.y: {}", start.y);
                while (start_y_temp + DASHED_LENGTH as f32) <= start_rel_y + rel_gap_first {
                    // Little line / stick
                    println!("start_y_temp + DASHED_LENGTH: {}, start_rel_y + rel_gap_first: {}", start_y_temp + DASHED_LENGTH as f32, start_rel_y + rel_gap_first);
                    draw_line_segment_mut(buffer,
                                          (start.x as f32, start_y_temp as f32),
                                          (start.x as f32, (start_y_temp + DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp += DASHED_LENGTH as f32 * (if (start_y_temp + DASHED_LENGTH as f32) < start_rel_y + rel_gap_first { 2.0 } else { 1.0 });
                }

            } else {
                while (start_y_temp - DASHED_LENGTH as f32) >= start_rel_y - rel_gap_second {
                    // Little line / stick
                    draw_line_segment_mut(buffer,
                                          (start.x as f32, start_y_temp as f32),
                                          (start.x as f32, (start_y_temp - DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp -= DASHED_LENGTH as f32 * 2.0;
                }

            }

            // Middle line
            if start.x < end.x {
                while (start_x_temp + DASHED_LENGTH as f32) <= end.x as f32 {
                    draw_line_segment_mut(buffer,
                                          (start_x_temp as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          ((start_x_temp + DASHED_LENGTH as f32) as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          general.colors.black);
                    start_x_temp += DASHED_LENGTH as f32 * 2.0;
                }
            } else {
                while (start_x_temp - DASHED_LENGTH as f32) >= end.x as f32 {
                    draw_line_segment_mut(buffer,
                                          (start_x_temp as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          ((start_x_temp - DASHED_LENGTH as f32) as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          general.colors.black);
                    start_x_temp -= DASHED_LENGTH as f32 * 2.0;
                }
            }

            // Little line / stick (SECOND)
            if end.y == base_first {
                while (start_y_temp - DASHED_LENGTH as f32) >= endy as f32 {
                    draw_line_segment_mut(buffer,
                                          (end.x as f32, start_y_temp as f32),
                                          (end.x as f32, (start_y_temp - DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp -= DASHED_LENGTH as f32 * 2.0;
                }
            } else {
                while (start_y_temp + DASHED_LENGTH as f32) <= endy as f32 {
                    draw_line_segment_mut(buffer,
                                          (end.x as f32, start_y_temp as f32),
                                          (end.x as f32, (start_y_temp + DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp += DASHED_LENGTH as f32 * 2.0;
                }
            }

            if is_in_first {
                rel_gap_first += REL_GAP_DISTANCE;
            } else {
                rel_gap_second += REL_GAP_DISTANCE;
            }
        }
        BorderType::None => {

        }
    }
    let mut ret: Vec<f32> = Vec::new();
    ret.push(rel_gap_first);
    ret.push(rel_gap_second);
    ret
}
