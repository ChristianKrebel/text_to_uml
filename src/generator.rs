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

pub struct RelationLayout {
    rel: Relation,
    start: XY,
    end: XY
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
const LETTER_WIDTH: u32 = 13;
const RELATION_GAP: u32 = 400;
const PADDING_LEFT: u32 = 4;
const PADDING_TOP: u32 = 2;
const RELATION_STICK: u32 = 50;

pub fn generate_pic(class_vec: &mut Vec<Class>, rel_vec: &mut Vec<Relation>) {
    println!("{:?}", rel_vec);
    let path = Path::new("output.png");

    // ------ Layouting all classes ------
    let mut class_layout_vec: Vec<ClassLayout> = Vec::new();
    let mut class_count = class_vec.len();

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

    /*println!("{}", greatest_height_first_half);
    println!("{}", greatest_height_second_half);*/

    let mut last_left_distance_uneven: u32 = 50;
    let mut last_left_distance_even: u32 = 50;

    for (i,c) in class_vec.iter().enumerate() {
        println!("LLDU: {}", last_left_distance_uneven);
        println!("LLDE: {}", last_left_distance_even);
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

    // Load the font
    let font_data = include_bytes!("../fonts/Roboto-Regular.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scales: Scales = Scales {
        one: Scale::uniform(32.0),
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
        draw_class(&mut imgbuf, &general, &font, &c, &class_layout_vec[i]);
    }

    // ------ Layouting all relations ------
    let mut rel_layout_vec: Vec<RelationLayout> = Vec::new();
    //rel_vec.sort_by_key(|x| x.from_class);

    println!("{:?}", rel_vec);
    // Durch alle Klassen
    for (i,c) in class_vec.iter().enumerate() {
        println!("cname: {}", c.class_name);
        let mut rel_starts: Vec<XY> = Vec::new();
        let mut rel_starts_stepsize: u32;
        let mut rels_indexes: Vec<usize> = Vec::new();

        // Durch alle Relationen
        for (index, rel) in rel_vec.iter().enumerate() {
            println!("rel index: {}", index);
            // Wenn Relation ausgeht, dann speichere Index der Relation
            if rel.from_class == c.class_name {
                println!("rel.from_class == c.class_name - index: {}", index);
                rels_indexes.push(index);
            }
        }
        println!("rels_indexes.len(): {}", rels_indexes.len());

        rel_starts_stepsize = class_layout_vec[i].width / (rels_indexes.len() as u32 + 1);
        let mut x_start: u32 = 0;
        let mut y_start: u32 = 0;
        if class_layout_vec[i].uneven {
            x_start = class_layout_vec[i].lb.x;
            y_start = class_layout_vec[i].lb.y;
        } else {
            x_start = class_layout_vec[i].lt.x;
            y_start = class_layout_vec[i].lt.y;
        }

        // Durch alle Indexe der Relationen, die aus der Klasse gehen^
        for index in rels_indexes {
            println!("crelation-index: {}", &index);

            // Durch alle Relationen
            for (l, rel) in rel_vec.iter().enumerate() {
                println!("relation to: {}", rel.to_class);
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
                            println!("class to: {}", c.class_name);
                            to_class_i = ci;
                        }
                    }
                    let mut x2_start: u32 = 0;
                    let mut y2_start: u32 = 0;
                    if class_layout_vec[to_class_i].uneven {
                        x2_start = class_layout_vec[to_class_i].lb.x;
                        y2_start = class_layout_vec[to_class_i].lb.y;
                    } else {
                        x2_start = class_layout_vec[to_class_i].lt.x;
                        y2_start = class_layout_vec[to_class_i].lt.y;
                    }

                    let mut xy2: XY = XY {
                        x: x2_start,
                        y: y2_start
                    };


                    /*let mut rl: RelationLayout = RelationLayout {
                        rel: rel,
                        start: xy1,
                        end: xy2
                    };*/

                    draw_rel(&mut imgbuf, &general, &font, &rel, &xy1, &xy2);
                }
            }
        }
    }
    // ------------
    //println!("vec länge: {}", rel_layout_vec.len());
    //println!("--- x:{}, y:{},  x:{}, y:{}", rel_layout_vec[0].start.x, rel_layout_vec[0].start.y, rel_layout_vec[0].end.x, rel_layout_vec[0].end.y);
    /*for (i, r) in rel_layout_vec.iter().enumerate() {
        println!("{}", i);
        draw_rel(&mut imgbuf, &general, &font, &r);
    }*/
    // ------------

    // Save the picture
    imgbuf.save(&path).unwrap();


    /*let mut img = DynamicImage::new_rgb8(imgx, imgy);

    // Construct a rectangle with top-left corner at (4, 5), width 6 and height 7.
    let rect = Rect::at(4, 5).of_size(6, 7);

    // Contains top-left point:
    assert_eq!(rect.left(), 4);
    assert_eq!(rect.top(), 5);
    assert!(rect.contains(rect.left(), rect.top()));

    // Contains bottom-right point, at (left + width - 1, top + height - 1):
    assert_eq!(rect.right(), 9);
    assert_eq!(rect.bottom(), 11);
    assert!(rect.contains(rect.right(), rect.bottom()));

    let mut rng = rand::thread_rng();
    let pos: (i32, i32) = (rng.gen_range(0, imgx as i32), rng.gen_range(0, imgy as i32));
    let color = Rgba([0, 0, 0, 1]);

    imageproc::drawing::draw_filled_circle_mut(&mut img, pos, 5, *color);*/

    // Save the image as “fractal.png”, the format is deduced from the path
    //imgbuf.save("test.png").unwrap();
    //img.save(&mut File::create(&Path::new("output.png")).unwrap(), image::PNG);

}

pub fn get_empty_relation() -> Relation {
    Relation {
        border_type: BorderType::None,
        arrow_type: RelationArrow::None,
        from_class: "".to_string(),
        from_class_card: "".to_string(),
        to_class: "".to_string(),
        to_class_card: "".to_string()
    }
}

pub fn draw_class(buffer: &mut image::RgbaImage, general: &General, font: &Font, class: &Class,
                  class_layout: &ClassLayout) {

    //let &buffer = &general.buffer;
    let x = general.imgxy.x;
    let y = general.imgxy.y;
    let colors = &general.colors;
    let scales = &general.scales;


    match class.class_type {
        ClassType::SimpleClass => {
            println!("width: {}, height: {}", &class_layout.width, &class_layout.height);

            // Outer borderline
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(
                    class_layout.lt.x as i32, class_layout.lt.y as i32).of_size(
                    class_layout.width, class_layout.height),
                colors.black);

            let mut height_to_write_at: u32 = class_layout.lt.y + PADDING_TOP;
            let mut has_stereotype: bool = if class.class_stereotype.is_empty() { false } else { true };

            // Draw name (and stereotype)
            if has_stereotype {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                    height_to_write_at, scales.two, &font, &class.class_stereotype);
                height_to_write_at += LINE_HEIGHT;
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                    height_to_write_at, scales.two, &font, &class.class_name);
                height_to_write_at += LINE_HEIGHT;
            } else {
                draw_text_mut(
                    buffer, colors.black, class_layout.lt.x + PADDING_LEFT,
                    height_to_write_at, scales.two, &font, &class.class_name);
                height_to_write_at += LINE_HEIGHT;
            }

            // Draw all other lines of text or just lines
            for (i, line) in class.content_lines.iter().enumerate() {
                let mut is_horizontal_line: bool = false;
                match class.content_decor[i] {
                    TextDecoration::None => {
                        println!("Textdeco: None");
                    }
                    TextDecoration::HorizontalLine => {
                        println!("Textdeco: HorizontalLine");
                        is_horizontal_line = true;
                    }
                    TextDecoration::Bold => {
                        println!("Textdeco: Bold");
                        // TODO
                    }
                    TextDecoration::Italic => {
                        println!("Textdeco: Italic");
                        // TODO
                    }
                    TextDecoration::BoldItalic => {
                        println!("Textdeco: BoldItalic");
                        // TODO
                    }
                    TextDecoration::Underlined => {
                        println!("Textdeco: Underlined");
                        // TODO
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
                        height_to_write_at, scales.two, &font, &line);
                }
                height_to_write_at += LINE_HEIGHT;
            }
        }
        ClassType::AbstractClass => {
            // TODO
        }
        ClassType::ActiveClass => {
            // TODO
        }
        ClassType::DashedBorderClass => {
            // TODO
        }
        ClassType::VarBorderClass => {
            // TODO
        }
        ClassType::None => {
            // TODO
        }
    }
}

pub fn draw_rel(buffer: &mut image::RgbaImage, general: &General, font: &Font, rel: &Relation, start: &XY, end: &XY) {
    println!("from: {}, from card: {}", rel.from_class, rel.from_class_card);
    println!("to: {}, to card: {}", rel.to_class, rel.to_class_card);
    draw_line_segment_mut(buffer,
                          (start.x as f32, start.y as f32),
                          (end.x as f32, end.y as f32),
                          general.colors.black);

}
