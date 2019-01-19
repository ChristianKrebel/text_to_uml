#[allow(unused_variables, unused_mut, unused)]
extern crate imageproc;
extern crate rand;
extern crate azul;
extern crate image;

use defines::*;

use self::imageproc::rect::*;
use self::imageproc::drawing::*;
use rusttype::{point, Font, Scale};

use std::str::*;
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
    white: image::Bgra<u8>,
    black: image::Bgra<u8>,
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
    imgxy: XY,
    colors: Colors,
    scales : Scales,
}




pub fn get_image(class_vec: &[Class], rel_vec: &[Relation]) -> (image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, (u32, u32)) {

}

pub fn draw_class(buffer: &mut image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, general: &General, fonts: &Vec<Font>, class: &Class,
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

            let mut height_to_write_at: u32 = class_layout.lt.y + ::PADDING_TOP;
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
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[1], &class.class_name);
                    height_to_write_at += ::LINE_HEIGHT;
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_stereotype);
                    height_to_write_at += ::LINE_HEIGHT;
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_name);
                    height_to_write_at += ::LINE_HEIGHT;
                }
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                    height_to_write_at, scales.two, &fonts[0], &class.class_name);
                height_to_write_at += ::LINE_HEIGHT;
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
                            height_to_write_at - class_layout.lt.y + (::LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32,
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32 +
                                                   (::LETTER_WIDTH as f32 * (line.len() as f32 - 1.0)),
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              general.colors.black);
                    }
                }
                height_to_write_at += ::LINE_HEIGHT;
            }
        }
        ClassType::AbstractClass => {
            // Outer borderline
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(
                    class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width, class_layout.height),
                colors.black);

            let mut height_to_write_at: u32 = class_layout.lt.y + ::PADDING_TOP;
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
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[1], &class.class_name);
                    height_to_write_at += ::LINE_HEIGHT;
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_stereotype);
                    height_to_write_at += ::LINE_HEIGHT;
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[0], &class.class_name);
                    height_to_write_at += ::LINE_HEIGHT;
                }
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                    height_to_write_at, scales.two, &fonts[0], &class.class_name);
                height_to_write_at += ::LINE_HEIGHT;
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
                            height_to_write_at - class_layout.lt.y + (::LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32,
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32 +
                                                   (::LETTER_WIDTH as f32 * (line.len() as f32 - 1.0)),
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              general.colors.black);
                    }
                }
                height_to_write_at += ::LINE_HEIGHT;
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
                    class_layout.lt.x as i32 + ::PADDING_LEFT as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width - ::ACTIVE_PADDING, class_layout.height),
                colors.black);

            let mut height_to_write_at: u32 = class_layout.lt.y + ::PADDING_TOP;
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
                        buffer, colors.black, class_layout.lt.x + ::ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[1], &class.class_name);
                    height_to_write_at += ::LINE_HEIGHT;
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[0], &class.class_stereotype);
                    height_to_write_at += ::LINE_HEIGHT;
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[0], &class.class_name);
                    height_to_write_at += ::LINE_HEIGHT;
                }
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + ::ACTIVE_PADDING,
                    height_to_write_at, scales.two, &fonts[0], &class.class_name);
                height_to_write_at += ::LINE_HEIGHT;
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
                            height_to_write_at - class_layout.lt.y + (::LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ::ACTIVE_PADDING) as f32,
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ::ACTIVE_PADDING) as f32 +
                                                   (::LETTER_WIDTH as f32 * (line.len() as f32 - 1.0)),
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              general.colors.black);
                    }
                }
                height_to_write_at += ::LINE_HEIGHT;
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

pub fn draw_rel(buffer: &mut image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, general: &General, fonts: &Vec<Font>, rel: &Relation,
                start: &XY, end: &XY, base_first: u32, rel_gap_first: f32, rel_gap_second: f32) -> Vec<f32> {

    println!("from: {}, from card: {}", rel.from_class, rel.from_class_card);
    println!("to: {}, to card: {}", rel.to_class, rel.to_class_card);

    let mut is_in_first: bool = if start.y == base_first { true } else { false };
    let mut start_rel_y: f32 = if is_in_first {(start.y + ::RELATION_STICK) as f32} else {(start.y - ::RELATION_STICK) as f32};
    let mut rel_gap_first = rel_gap_first;
    let mut rel_gap_second = rel_gap_second;

    // Arrows
    match rel.arrow_type {
        RelationArrow::Arrow => {
            if end.y == base_first {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ::ARROW_SIZE as f32, end.y as f32 + ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ::ARROW_SIZE as f32, end.y as f32 + ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
            } else {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ::ARROW_SIZE as f32, end.y as f32 - ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ::ARROW_SIZE as f32, end.y as f32 - ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
            }

        }
        RelationArrow::TriangleEmpty => {
            if end.y == base_first {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ::ARROW_SIZE as f32, end.y as f32 + ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ::ARROW_SIZE as f32, end.y as f32 + ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ::ARROW_SIZE as f32, end.y as f32 + ::ARROW_SIZE as f32),
                                      (end.x as f32 - ::ARROW_SIZE as f32, end.y as f32 + ::ARROW_SIZE as f32),
                                      general.colors.black);
            } else {
                draw_line_segment_mut(buffer,
                                      (end.x as f32 - ::ARROW_SIZE as f32, end.y as f32 - ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ::ARROW_SIZE as f32, end.y as f32 - ::ARROW_SIZE as f32),
                                      (end.x as f32, end.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (end.x as f32 + ::ARROW_SIZE as f32, end.y as f32 - ::ARROW_SIZE as f32),
                                      (end.x as f32 - ::ARROW_SIZE as f32, end.y as f32 - ::ARROW_SIZE as f32),
                                      general.colors.black);
            }
        }
        RelationArrow::DiamondEmpty => {
            if is_in_first {
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ::ARROW_SIZE as f32, start.y as f32 + ::ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ::ARROW_SIZE as f32, start.y as f32 + ::ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ::ARROW_SIZE as f32, start.y as f32 + ::ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32 + (::ARROW_SIZE*2) as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ::ARROW_SIZE as f32, start.y as f32 + ::ARROW_SIZE as f32),
                                      (start.x as f32, (start.y + ::ARROW_SIZE * 2) as f32),
                                      general.colors.black);
            } else {
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ::ARROW_SIZE as f32, start.y as f32 - ::ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ::ARROW_SIZE as f32, start.y as f32 - ::ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 - ::ARROW_SIZE as f32, start.y as f32 - ::ARROW_SIZE as f32),
                                      (start.x as f32, start.y as f32 - (::ARROW_SIZE*2) as f32),
                                      general.colors.black);
                draw_line_segment_mut(buffer,
                                      (start.x as f32 + ::ARROW_SIZE as f32, start.y as f32 - ::ARROW_SIZE as f32),
                                      (start.x as f32, (start.y - ::ARROW_SIZE * 2) as f32),
                                      general.colors.black);
            }
        }
        RelationArrow::DiamondFilled => {
            if is_in_first {
                let mut p1: Point<i32> = Point::new(start.x as i32, start.y as i32);
                let mut p2: Point<i32> = Point::new(start.x as i32 - ::ARROW_SIZE as i32, start.y as i32 + ::ARROW_SIZE as i32);
                let mut p3: Point<i32> = Point::new(start.x as i32, start.y as i32 + (::ARROW_SIZE * 2) as i32);
                let mut p4: Point<i32> = Point::new(start.x as i32 + ::ARROW_SIZE as i32, start.y as i32 + ::ARROW_SIZE as i32);
                draw_convex_polygon_mut(buffer, &[p1, p2, p3, p4], general.colors.black);
            } else {
                let mut p1: Point<i32> = Point::new(start.x as i32, start.y as i32);
                let mut p2: Point<i32> = Point::new(start.x as i32 - ::ARROW_SIZE as i32, start.y as i32 - ::ARROW_SIZE as i32);
                let mut p3: Point<i32> = Point::new(start.x as i32, start.y as i32 - (::ARROW_SIZE * 2) as i32);
                let mut p4: Point<i32> = Point::new(start.x as i32 + ::ARROW_SIZE as i32, start.y as i32 - ::ARROW_SIZE as i32);
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
                    starty += (::ARROW_SIZE*2);
                } else {
                    starty -= (::ARROW_SIZE*2);
                }
            };
            if rel.arrow_type == RelationArrow::TriangleEmpty {
                if end.y == base_first {
                    endy += ::ARROW_SIZE;
                } else {
                    endy -= ::ARROW_SIZE;
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
                    buffer, general.colors.black, start.x as u32 + ::CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + ::CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.from_class_card);
            }
            if !rel.to_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, end.x as u32 + ::CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + ::CARD_DIST as f32) as u32,
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
                rel_gap_first += ::REL_GAP_DISTANCE;
            } else {
                rel_gap_second += ::REL_GAP_DISTANCE;
            }
        }
        BorderType::Dashed => {
            let mut endy = end.y;
            if rel.arrow_type == RelationArrow::TriangleEmpty {
                if end.y == base_first {
                    endy += ::ARROW_SIZE;
                } else {
                    endy -= ::ARROW_SIZE;
                }
            };

            let mut start_y_temp = start.y as f32;
            let mut start_x_temp = start.x as f32;
            // Card. / multiplicities
            if !rel.from_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, start.x as u32 + ::CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + ::CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.from_class_card);
            }
            if !rel.to_class_card.is_empty() {
                draw_text_mut(
                    buffer, general.colors.black, end.x as u32 + ::CARD_DIST as u32,
                    (start_rel_y +
                        if is_in_first { rel_gap_first } else { -rel_gap_second } + ::CARD_DIST as f32) as u32,
                    general.scales.one, &fonts[0], &rel.to_class_card);
            }
            // Little line / stick (FIRST)
            if is_in_first {
                println!("start.y: {}", start.y);
                while (start_y_temp + ::DASHED_LENGTH as f32) <= start_rel_y + rel_gap_first {
                    // Little line / stick
                    println!("start_y_temp + ::DASHED_LENGTH: {}, start_rel_y + rel_gap_first: {}", start_y_temp + ::DASHED_LENGTH as f32, start_rel_y + rel_gap_first);
                    draw_line_segment_mut(buffer,
                                          (start.x as f32, start_y_temp as f32),
                                          (start.x as f32, (start_y_temp + ::DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp += ::DASHED_LENGTH as f32 * (if (start_y_temp + ::DASHED_LENGTH as f32) < start_rel_y + rel_gap_first { 2.0 } else { 1.0 });
                }

            } else {
                while (start_y_temp - ::DASHED_LENGTH as f32) >= start_rel_y - rel_gap_second {
                    // Little line / stick
                    draw_line_segment_mut(buffer,
                                          (start.x as f32, start_y_temp as f32),
                                          (start.x as f32, (start_y_temp - ::DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp -= ::DASHED_LENGTH as f32 * 2.0;
                }

            }

            // Middle line
            if start.x < end.x {
                while (start_x_temp + ::DASHED_LENGTH as f32) <= end.x as f32 {
                    draw_line_segment_mut(buffer,
                                          (start_x_temp as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          ((start_x_temp + ::DASHED_LENGTH as f32) as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          general.colors.black);
                    start_x_temp += ::DASHED_LENGTH as f32 * 2.0;
                }
            } else {
                while (start_x_temp - ::DASHED_LENGTH as f32) >= end.x as f32 {
                    draw_line_segment_mut(buffer,
                                          (start_x_temp as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          ((start_x_temp - ::DASHED_LENGTH as f32) as f32, start_rel_y as f32 + (
                                              if is_in_first { rel_gap_first } else { -rel_gap_second })),
                                          general.colors.black);
                    start_x_temp -= ::DASHED_LENGTH as f32 * 2.0;
                }
            }

            // Little line / stick (SECOND)
            if end.y == base_first {
                while (start_y_temp - ::DASHED_LENGTH as f32) >= endy as f32 {
                    draw_line_segment_mut(buffer,
                                          (end.x as f32, start_y_temp as f32),
                                          (end.x as f32, (start_y_temp - ::DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp -= ::DASHED_LENGTH as f32 * 2.0;
                }
            } else {
                while (start_y_temp + ::DASHED_LENGTH as f32) <= endy as f32 {
                    draw_line_segment_mut(buffer,
                                          (end.x as f32, start_y_temp as f32),
                                          (end.x as f32, (start_y_temp + ::DASHED_LENGTH as f32) as f32),
                                          general.colors.black);
                    start_y_temp += ::DASHED_LENGTH as f32 * 2.0;
                }
            }

            if is_in_first {
                rel_gap_first += ::REL_GAP_DISTANCE;
            } else {
                rel_gap_second += ::REL_GAP_DISTANCE;
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
