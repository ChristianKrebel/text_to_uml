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
struct General {
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
        buffer: imgbuf,
        imgxy: xy,
        colors: colors,
        scales: scales,
    };

    // ------ DRAW -------
    for c in class_vec.iter() {
        if c.class_type == ClassType::SimpleClass {
            draw_class(&general, &font, &c);
        }
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

pub fn draw_class(general: &General, font: &Font, class: &Class) {

    let buffer = &general.buffer;
    let x = general.imgxy.x;
    let y = general.imgxy.y;
    let colors = &general.colors;
    let scales = &general.scales;

    draw_filled_rect_mut(
        buffer, imageproc::rect::Rect::at(0, 0).of_size(x, y),
        &colors.white);

    match class.class_type {
        ClassType::SimpleClass => {
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(75, 75).of_size(209, 30),
                &colors.black);
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(75, 75).of_size(209, 60),
                &colors.black);                              // height +30, width = längste Stringlänge * 11
            draw_hollow_rect_mut(
                buffer, imageproc::rect::Rect::at(75, 75).of_size(209, 90),
                &colors.black);
            draw_text_mut(
                buffer, &colors.black, 80, 77, scales.one, &font, &class.class_name); // y + 30
            for line in class.content_lines.iter() {
                draw_text_mut(
                    buffer, &colors.black, 80, 107, scales.two, &font, &line);
            }
        }
    }
}