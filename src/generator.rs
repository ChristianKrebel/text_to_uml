#[allow(unused_variables, unused_mut, unused)]

extern crate image;
extern crate imageproc;
extern crate rand;

use self::imageproc::rect::*;
use self::imageproc::drawing::*;
use rusttype::{point, Font, Scale};

use std::fs::File;
use std::path::Path;
use self::rand::Rng;

use self::image::{DynamicImage, GenericImage, Pixel, Rgba, RgbaImage, ImageFormat};


pub fn generate_pic(obj_list: &Vec<Struct>) {

    let imgx = 1080;
    let imgy = 1080;

    //let scalex = 4.0 / imgx as f32;
    //let scaley = 4.0 / imgy as f32;


    let path = Path::new("output.png");

    let white = Rgba([255u8, 255u8, 255u8, 255u8]);
    let red = Rgba([255u8, 0u8, 0u8, 127u8]);
    let blue = Rgba([0u8, 0u8, 255u8, 127u8]);
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);

    // Load the font
    let font_data = include_bytes!("../fonts/Roboto-Regular.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(32.0);
    let scale2 = Scale::uniform(26.0);



    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::RgbaImage::new(imgx, imgy);
    //let mut img = RgbaImage::new(imgx, imgy);
    //let mut img = image::open(path).unwrap();

    // ------ DRAW -------
    for i in &obj_list {
        if i.type == "class" {
            draw_class(&imgbuf, &i);
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

pub fn draw_class(&mut buffer: image::RgbaImage, &struc: Struct) {
    draw_filled_rect_mut(
        &mut imgbuf, imageproc::rect::Rect::at(0, 0).of_size(imgx, imgy),
        white);
    draw_hollow_rect_mut(
        &mut imgbuf, imageproc::rect::Rect::at(75, 75).of_size(209, 30),
        black);
    draw_hollow_rect_mut(
        &mut imgbuf, imageproc::rect::Rect::at(75, 75).of_size(209, 60),
        black);                              // height +30, width = längste Stringlänge * 11
    draw_hollow_rect_mut(
        &mut imgbuf, imageproc::rect::Rect::at(75, 75).of_size(209, 90),
        black);
    draw_text_mut(
        &mut imgbuf, black, 80, 77, scale2, &font, "Test"); // y + 30
    draw_text_mut(
        &mut imgbuf, black, 80, 107, scale2, &font, "- Farbe: Color");
    draw_text_mut(
        &mut imgbuf, black, 80, 137, scale2, &font, "+ getFarbe(): Color");

}