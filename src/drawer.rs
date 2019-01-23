#[allow(unused_variables, unused_mut, unused)]
extern crate imageproc;
extern crate rand;
extern crate azul;
extern crate image;

use defines::*;
use generator;

use self::imageproc::rect::*;
use self::imageproc::drawing::*;
use rusttype::{point, Font, Scale};

use std::str::*;
use std::mem;
use std::num::Wrapping;
use self::rand::Rng;

use self::image::{DynamicImage, GenericImage, Pixel, Rgba, RgbaImage, ImageFormat};


pub fn get_image(mut model: ModelContainer) -> (image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, (u32, u32)) {

    let (mut dim_x, mut dim_y) = (1, 1);
    let mut layout_vec = Vec::new();
    let mut layout2_vec = Vec::new();


    // Get the layout vec and picture dimensions
    match model.model_type {
        ModelType::ClassModel => {
            let cm = model.class_model.unwrap();
            let (a, b, c, d) = generator::generate_class_model_layout(
                &cm.classes,
                &cm.relations
            );
            layout_vec = a;
            layout2_vec = b;
            dim_x = c;
            dim_y = d;
            model = ModelContainer { model_type: ModelType::ClassModel, class_model: Some(cm), object_model:None, package_model:None, use_case_model:None };
        }
        ModelType::ObjectModel => {
            let om = model.object_model.unwrap();
            let (a, b, c, d) = generator::generate_object_model_layout(
                &om.objects,
                &om.links
            );
            layout_vec = a;
            layout2_vec = b;
            dim_x = c;
            dim_y = d;
            model = ModelContainer { model_type: ModelType::ObjectModel, class_model:None, object_model: Some(om), package_model:None, use_case_model:None };
        }
        ModelType::PackageModel => {
            // TODO
        }
        ModelType::UseCaseModel => {
            // TODO
        }
        ModelType::None => {
            // Done
        }
    }

    println!("x: {}, y:{}", dim_x, dim_y);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut img_buf = image::DynamicImage::new_rgba8(dim_x, dim_y).to_bgra();

    // Colors
    let colors: Colors = Colors {
        white: image::Bgra([255u8,255u8,255u8,255u8]),
        black: image::Bgra([0u8, 0u8, 0u8, 255u8]),
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
        one: Scale::uniform(16.0),
        two: Scale::uniform(26.0),
    };

    // The Dimensions
    let xy: XY = XY {
        x: dim_x,
        y: dim_y,
    };

    // Most important general info
    let general: General = General {
        imgxy: xy,
        colors: colors,
        scales: scales,
    };

    // Draw background
    draw_filled_rect_mut(
        &mut img_buf, imageproc::rect::Rect::at(0, 0).of_size(general.imgxy.x, general.imgxy.y),
        general.colors.white);

    // ------ DRAW ------
    match model.model_type {
        ModelType::ClassModel => {
            let cm = model.class_model.unwrap();
            for (i, c) in cm.classes.iter().enumerate() {
                draw_class(&mut img_buf, &general, &font_vec, &c, &layout_vec[i]);
            }
            for (i, r) in cm.relations.iter().enumerate() {
                draw_rel(&mut img_buf, &general,
                         &font_vec,
                         &r,
                         &layout2_vec[i].start,
                         &layout2_vec[i].end,
                         layout2_vec[i].base_first,
                         layout2_vec[i].gap_first,
                         layout2_vec[i].gap_second)
            }
        }
        ModelType::ObjectModel => {
            let om = model.object_model.unwrap();
            for (i, o) in om.objects.iter().enumerate() {
                draw_object(&mut img_buf, &general, &font_vec, &o, &layout_vec[i]);
            }
            for (i, l) in om.links.iter().enumerate() {
                draw_link(&mut img_buf, &general,
                         &font_vec,
                         &l,
                         &layout2_vec[i].start,
                         &layout2_vec[i].end,
                         layout2_vec[i].base_first,
                         layout2_vec[i].gap_first,
                         layout2_vec[i].gap_second)
            }
        }
        ModelType::PackageModel => {
            // TODO
        }
        ModelType::UseCaseModel => {
            // TODO
        }
        ModelType::None => {
            // Done
        }
    }

    //---------

    (img_buf, (dim_x, dim_y))
}

pub fn draw_class(buffer: &mut image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, general: &General, fonts: &Vec<Font>, class: &Class,
                  class_layout: &BoxLayout) {

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
            for (i, line) in class.lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                let mut is_underlined: bool = false;
                match class.lines[i].decor {
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
                if is_horizontal_line || line.content.is_empty() || line.content == "-" {
                    draw_hollow_rect_mut(
                        buffer, imageproc::rect::Rect::at(
                            class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                            class_layout.width,
                            height_to_write_at - class_layout.lt.y + (::LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line.content);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32,
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32 +
                                                   (::LETTER_WIDTH as f32 * (line.content.len() as f32 - 1.0)),
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
            for (i, line) in class.lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                let mut is_underlined: bool = false;
                match class.lines[i].decor {
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
                if is_horizontal_line || line.content.is_empty() || line.content == "-" {
                    draw_hollow_rect_mut(
                        buffer, imageproc::rect::Rect::at(
                            class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                            class_layout.width,
                            height_to_write_at - class_layout.lt.y + (::LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::PADDING_LEFT,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line.content);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32,
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ::PADDING_LEFT) as f32 +
                                                   (::LETTER_WIDTH as f32 * (line.content.len() as f32 - 1.0)),
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
            for (i, line) in class.lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                let mut is_underlined: bool = false;
                match class.lines[i].decor {
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
                if is_horizontal_line || line.content.is_empty() || line.content == "-" {
                    draw_hollow_rect_mut(
                        buffer, imageproc::rect::Rect::at(
                            class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                            class_layout.width,
                            height_to_write_at - class_layout.lt.y + (::LINE_HEIGHT / 2)),
                        colors.black);
                } else {
                    draw_text_mut(
                        buffer, colors.black, class_layout.lt.x + ::ACTIVE_PADDING,
                        height_to_write_at, scales.two, &fonts[deco_font as usize], &line.content);
                    if is_underlined {
                        draw_line_segment_mut(buffer,
                                              ((class_layout.lt.x + ::ACTIVE_PADDING) as f32,
                                               height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                                              ((class_layout.lt.x + ::ACTIVE_PADDING) as f32 +
                                                   (::LETTER_WIDTH as f32 * (line.content.len() as f32 - 1.0)),
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
                start: &XY, end: &XY, base_first: u32, rel_gap_first: f32, rel_gap_second: f32) {

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
        }
        BorderType::None => {

        }
    }
}


pub fn draw_object(buffer: &mut image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, general: &General,
                   fonts: &Vec<Font>, object: &Object, object_layout: &BoxLayout) {

    let x = general.imgxy.x;
    let y = general.imgxy.y;
    let colors = &general.colors;
    let scales = &general.scales;


    // Outer borderline
    draw_hollow_rect_mut(
        buffer, imageproc::rect::Rect::at(
            object_layout.lt.x as i32, object_layout.lt.y as i32).of_size(
            object_layout.width, object_layout.height),
        colors.black);

    let mut height_to_write_at: u32 = object_layout.lt.y + ::PADDING_TOP;

    // Draw name
    draw_text_mut(
        buffer, colors.black, object_layout.lt.x + ::PADDING_LEFT,
        height_to_write_at, scales.two, &fonts[0], &object.object_title);

    // Underline it
    draw_line_segment_mut(buffer,
                          ((object_layout.lt.x + ::PADDING_LEFT) as f32,
                           height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                          ((object_layout.lt.x + ::PADDING_LEFT) as f32 +
                               (::LETTER_WIDTH as f32 * (object.object_title.len() as f32 - 1.0)),
                           height_to_write_at as f32 + ::LINE_HEIGHT as f32 - 6.0),
                          general.colors.black);

    height_to_write_at += ::LINE_HEIGHT;

    // Draw all other lines of text
    for (i, line) in object.content_lines.iter().enumerate() {
        draw_text_mut(
            buffer, colors.black, object_layout.lt.x + ::PADDING_LEFT,
            height_to_write_at, scales.two, &fonts[0], &line);

        height_to_write_at += ::LINE_HEIGHT;
    }
}


pub fn draw_link(buffer: &mut image::ImageBuffer<image::Bgra<u8>, Vec<u8>>, general: &General,
                 fonts: &Vec<Font>, link: &Link, start: &XY, end: &XY, base_first: u32,
                 link_gap_first: f32, link_gap_second: f32) {

    println!("from: {}, from role: {}", link.from_object, link.from_object_role);
    println!("to: {}, to role: {}", link.to_object, link.to_object_role);

    let mut is_in_first: bool = if start.y == base_first { true } else { false };
    let mut start_link_y: f32 = if is_in_first {(start.y + ::RELATION_STICK) as f32} else {(start.y - ::RELATION_STICK) as f32};
    let mut link_gap_first = link_gap_first;
    let mut link_gap_second = link_gap_second;

    // Lines
    let mut starty = start.y;
    let mut endy = end.y;
    // Little line / stick
    draw_line_segment_mut(buffer,
                          (start.x as f32, starty as f32),
                          (start.x as f32, start_link_y as f32 + (
                              if is_in_first { link_gap_first } else { -link_gap_second })),
                          general.colors.black);
    // Card. / multiplicities / roles
    if !link.from_object_role.is_empty() {
        draw_text_mut(
            buffer, general.colors.black, start.x as u32 + ::CARD_DIST as u32,
            (start_link_y +
                if is_in_first { link_gap_first } else { -link_gap_second } + ::CARD_DIST as f32) as u32,
            general.scales.one, &fonts[0], &link.from_object_role);
    }
    // link name
    if !link.link_name.is_empty() {

        // Draw text
        let link_x = ((if start.x < end.x
            {
                start.x + (end.x - start.x)/2
            }
            else {
                end.x + (start.x - end.x)/2
            }) - link.link_name.len() as u32 /2 as u32 * ::LETTER_WIDTH_ACCURATE as u32) as u32;

        let link_y = (start_link_y +
            if is_in_first { link_gap_first }
                else { -link_gap_second } + ::ROLE_NAME_DIST as f32 - ::LINE_HEIGHT as f32) as u32;

        draw_text_mut(
            buffer, general.colors.black,
            link_x,
            link_y,
            general.scales.one,
            &fonts[0],
            &link.link_name);

        // Draw arrow
        let mut p1: Point<i32>;
        let mut p2: Point<i32>;
        let mut p3: Point<i32>;

        if start.x < end.x {
            p1 = Point::new((link_x - ::CARD_DIST)as i32, (link_y + 14/2) as i32);
            p2 = Point::new((link_x - ::CARD_DIST - ::ROLE_NAME_ARROW_SIZE)as i32, (link_y + 14) as i32);
            p3 = Point::new((link_x - ::CARD_DIST - ::ROLE_NAME_ARROW_SIZE)as i32, (link_y) as i32);
        } else {
            p1 = Point::new((link_x - ::CARD_DIST - ::ROLE_NAME_ARROW_SIZE)as i32, (link_y + 14/2) as i32);
            p2 = Point::new((link_x - ::CARD_DIST)as i32, (link_y + 14) as i32);
            p3 = Point::new((link_x - ::CARD_DIST)as i32, (link_y) as i32);
        }

        draw_convex_polygon_mut(buffer, &[p1, p2, p3], general.colors.black);
    }
    if !link.to_object_role.is_empty() {
        draw_text_mut(
            buffer, general.colors.black, end.x as u32 + ::CARD_DIST as u32,
            (start_link_y +
                if is_in_first { link_gap_first } else { -link_gap_second } + ::CARD_DIST as f32) as u32,
            general.scales.one, &fonts[0], &link.to_object_role);
    }
    // Big lines
    draw_line_segment_mut(buffer,
                          (start.x as f32, start_link_y as f32 + (
                              if is_in_first { link_gap_first } else { -link_gap_second })),
                          (end.x as f32, start_link_y as f32 + (
                              if is_in_first { link_gap_first } else { -link_gap_second })),
                          general.colors.black);
    draw_line_segment_mut(buffer,
                          (end.x as f32, start_link_y as f32 + (
                              if is_in_first { link_gap_first } else { -link_gap_second })),
                          (end.x as f32, endy as f32),
                          general.colors.black);
}
