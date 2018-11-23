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
use self::rand::Rng;

use self::image::{DynamicImage, GenericImage, Pixel, Rgba, RgbaImage, ImageFormat};

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
struct XY {
    x: u32,
    y: u32,
}
pub struct General {
    buffer: image::RgbaImage,
    imgxy: XY,
    colors: Colors,
    scales : Scales,
}


pub fn generate_pic(class_vec: &mut Vec<Class>, rel_vec: &mut Vec<Relation>) {

    let path = Path::new("output.png");

    let xy: XY = XY {
        x: 1080,
        y: 1080,
    };

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
    //let mut img = RgbaImage::new(imgx, imgy);
    //let mut img = image::open(path).unwrap();

    let general: General = General {
        buffer: image::RgbaImage::new(xy.x, xy.y),
        imgxy: xy,
        colors: colors,
        scales: scales,
    };

    draw_filled_rect_mut(
        &mut imgbuf, imageproc::rect::Rect::at(0, 0).of_size(general.imgxy.x, general.imgxy.y),
        general.colors.white);

    let mut i:i32 = 0;
    let mut e:i32= 0;
    let mut add_to_i: i32 = 0;
    // ------ DRAW -------
    for c in class_vec.iter() {
        add_to_i = draw_class(&mut imgbuf, &general, &font, &c, i, e);
        i += add_to_i + 50;
    }

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

pub fn draw_class(buffer: &mut image::RgbaImage, general: &General, font: &Font, class: &Class, i:i32, e:i32) -> i32 {

    //let &buffer = &general.buffer;
    let x = general.imgxy.x;
    let y = general.imgxy.y;
    let colors = &general.colors;
    let scales = &general.scales;

    let mut width = 0;
    for line in class.content_lines.iter() {
        if line.len() > width {
            width = line.len();
        }
    }
    if class.class_name.len() > width {
        width = class.class_name.len();
    }
    width *= 11;
    let mut lt = x;
    let mut rt = x + width as u32;
    let mut lb: u32;
    let mut rb: u32;



    match class.class_type {
        ClassType::SimpleClass => {
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(i, e).of_size(209, 30),
                colors.black);
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(i, e).of_size(209, 60),
                colors.black);                              // height +30, width = längste Stringlänge * 11
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(i, e).of_size(209, 90),
                colors.black);
            let mut j = i as u32 + 5;
            let mut k:u32 = 0;
            let mut name_length = class.class_name.len() as u32;
            draw_text_mut(
                buffer, colors.black, j + 5, k, scales.one, &font, &class.class_name); // y + 30
            for line in class.content_lines.iter() {
                k += 30;
                draw_text_mut(
                    buffer, colors.black, j, k, scales.two, &font, &line);
            }
        }
        ClassType::AbstractClass => {

        }
        ClassType::ActiveClass => {

        }
        ClassType::DashedBorderClass => {

        }
        ClassType::VarBorderClass => {

        }
        ClassType::None => {

        }
    }
    return width as i32;
}