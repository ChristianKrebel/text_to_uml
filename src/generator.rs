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


pub fn generate_class_model_layout(class_vec: &[Class], rel_vec: &[Relation]) -> (Vec<BoxLayout>, Vec<LineLayout>, u32, u32) {

    // ------ Layouting all classes ------
    let mut class_layout_vec: Vec<BoxLayout> = Vec::new();
    let mut class_count = class_vec.len();

    // calc distance between upper and lower classes
    let RELATION_GAP: u32 = ((::RELATION_STICK * 2) as f32 + (rel_vec.len() as f32 + 1.0) * ::REL_GAP_DISTANCE) as u32;

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

    greatest_height_first_half *= ::LINE_HEIGHT;
    greatest_height_second_half *= ::LINE_HEIGHT;
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
        greatest_width *= ::LETTER_WIDTH;

        let mut height: u32 = 0;

            if !c.class_name.is_empty() {
                height += 1;
            }
            if !c.class_stereotype.is_empty() {
                height += 1;
            }
            height += c.content_lines.len() as u32;

        height *= ::LINE_HEIGHT;

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


        let class_layout: BoxLayout = BoxLayout {
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


    // ------ Layouting all relations ------


    let mut rel_gap_first = ::REL_GAP_DISTANCE;
    let mut rel_gap_second = ::REL_GAP_DISTANCE;

    let mut rel_layout_vec: Vec<LineLayout> = Vec::new();

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

                    let xy1y = xy1.y;

                    let rel_layout: LineLayout = LineLayout {
                        start: xy1,
                        end: xy2,
                        base_first: base_line_first_half,
                        gap_first: rel_gap_first,
                        gap_second: rel_gap_second
                    };

                    rel_layout_vec.push(rel_layout);

                    if xy1y == base_line_first_half {
                        rel_gap_first += ::REL_GAP_DISTANCE;
                    } else {
                        rel_gap_second += ::REL_GAP_DISTANCE;
                    }
                }
            }
        }
    }

    (class_layout_vec, rel_layout_vec, greatest_last_left_distance, top_line_second_half + greatest_height_second_half + 50)
}

// TODO
pub fn generate_object_model_layout(object_vec: &[Object], link_vec: &[Link]) -> (Vec<BoxLayout>, Vec<LineLayout>, u32, u32) {

    // ------ Layouting all objects ------
    let mut object_layout_vec: Vec<BoxLayout> = Vec::new();
    let mut object_count = object_vec.len();

    // calc distance between upper and lower objects
    let RELATION_GAP: u32 = ((::RELATION_STICK * 2) as f32 + (link_vec.len() as f32 + 1.0) * ::REL_GAP_DISTANCE) as u32;

    // calc heights for upper half of objects (uneven)
    let mut greatest_height_first_half: u32 = 0;
    for (i,o) in object_vec.iter().enumerate() {
        let mut greatest_height: u32 = 0;
        if i % 2 != 0 {
            if !o.object_title.is_empty() {
                greatest_height += 1;
            }
            greatest_height += o.content_lines.len() as u32;
        }
        if greatest_height > greatest_height_first_half {
            greatest_height_first_half = greatest_height;
        }
    }

    // calc heights for lower half of objects (even)
    let mut greatest_height_second_half: u32 = 0;
    for (i,o) in object_vec.iter().enumerate() {
        let mut greatest_height: u32 = 0;
        if i % 2 == 0 {
            if !o.object_title.is_empty() {
                greatest_height += 1;
            }
            greatest_height += o.content_lines.len() as u32;
        }
        if greatest_height > greatest_height_second_half {
            greatest_height_second_half = greatest_height;
        }
    }

    greatest_height_first_half *= ::LINE_HEIGHT;
    greatest_height_second_half *= ::LINE_HEIGHT;
    let mut base_line_first_half: u32 = greatest_height_first_half + 50;
    let mut top_line_second_half: u32 = if object_count == 1
        {base_line_first_half} else {base_line_first_half + RELATION_GAP};


    let mut last_left_distance_uneven: u32 = 50;
    let mut last_left_distance_even: u32 = 50;

    for (i,o) in object_vec.iter().enumerate() {

        let mut greatest_width: u32 = 0;
        for line in o.content_lines.iter() {
            if line.len() as u32 > greatest_width {
                greatest_width = line.len() as u32;
            }
        }
        if !o.object_title.is_empty() {
            if o.object_title.len() as u32 > greatest_width {
                greatest_width = o.object_title.len() as u32;
            }
        }
        greatest_width *= ::LETTER_WIDTH;

        let mut height: u32 = 0;

        if !o.object_title.is_empty() {
            height += 1;
        }
        height += o.content_lines.len() as u32;

        height *= ::LINE_HEIGHT;

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


        let object_layout: BoxLayout = BoxLayout {
            lt: lt,
            rt: rt,
            lb: lb,
            rb: rb,
            height: height,
            width: greatest_width,
            uneven: uneven,
        };
        object_layout_vec.push(object_layout);
        if i % 2 != 0 {
            last_left_distance_uneven += &greatest_width + 200;
        } else {
            last_left_distance_even += &greatest_width + 200;
        }
    }

    // ------------



    // Calc picture bounds
    let mut greatest_last_left_distance: u32 = if last_left_distance_uneven > last_left_distance_even
        {last_left_distance_uneven - 150} else {last_left_distance_even - 150};
    let xy: XY = XY {
        x: greatest_last_left_distance,
        y: top_line_second_half + greatest_height_second_half + 50,
    };


    // ------ Layouting all Links ------


    let mut link_gap_first = ::REL_GAP_DISTANCE;
    let mut link_gap_second = ::REL_GAP_DISTANCE;

    let mut link_layout_vec: Vec<LineLayout> = Vec::new();

    let mut all_to_object_links_vec: Vec<Vec<bool>> = Vec::new();
    for (i, o) in object_vec.iter().enumerate() {
        let mut empty_vec: Vec<bool> = Vec::new();
        empty_vec.push(true);
        all_to_object_links_vec.push(empty_vec);
    }


    for (i, o) in object_vec.iter().enumerate() {
        let mut to_object_links_vec: Vec<bool> = Vec::new();
        // Durch alle Links
        for (index, link) in link_vec.iter().enumerate() {
            // Wenn Link eingeht, dann speichere Index des Links
            if link.to_object == o.object_intern_name {
                to_object_links_vec.push(false);
            }
        }
        all_to_object_links_vec[i] = to_object_links_vec;
    }


    // Durch alle Objekte
    for (i,o) in object_vec.iter().enumerate() {
        let mut link_starts: Vec<XY> = Vec::new();
        let mut link_starts_stepsize: u32;
        let mut link_ends_stepsize: u32;
        let mut links_indexes: Vec<usize> = Vec::new();
        let mut links_indexes2: Vec<usize> = Vec::new();

        // Durch alle Links
        for (index, link) in link_vec.iter().enumerate() {
            // Wenn Link ausgeht, dann speichere Index des Links
            if link.from_object == o.object_intern_name {
                links_indexes.push(index);
            }
            // Wenn Links eingeht, dann speichere Index der Relation
            if link.to_object == o.object_intern_name {
                links_indexes2.push(index);
            }
        }


        link_starts_stepsize = (object_layout_vec[i].width/2) / (links_indexes.len() as u32 + 1);


        let mut x_start: u32 = 0;
        let mut y_start: u32 = 0;
        let mut x_end: u32 = 0;
        let mut y_end: u32 = 0;

        if object_layout_vec[i].uneven {
            x_start = object_layout_vec[i].lb.x;
            y_start = object_layout_vec[i].lb.y;
        } else {
            x_start = object_layout_vec[i].lt.x;
            y_start = object_layout_vec[i].lt.y;
        }



        // Durch alle Indexe der Links, die aus dem Objekt gehen^
        for index in links_indexes {

            // Durch alle Links
            for (l, link) in link_vec.iter().enumerate() {
                // Wenn Index der Links des Objekts dem Index der durchlaufenden Links ist
                if index == l {

                    x_start += link_starts_stepsize;
                    let mut xy1: XY = XY {
                        x: x_start,
                        y: y_start
                    };

                    let mut to_object_i: usize = 0;
                    for (ci, o) in object_vec.iter().enumerate() {
                        if o.object_intern_name == link.to_object {
                            to_object_i = ci;
                        }
                    }
                    link_ends_stepsize = (object_layout_vec[to_object_i].width/2) / (all_to_object_links_vec[to_object_i].len() as u32 + 1);
                    if object_layout_vec[to_object_i].uneven {
                        x_end = object_layout_vec[to_object_i].lb.x + (object_layout_vec[to_object_i].width/2);
                        y_end = object_layout_vec[to_object_i].lb.y;
                    } else {
                        x_end = object_layout_vec[to_object_i].lt.x + (object_layout_vec[to_object_i].width/2);
                        y_end = object_layout_vec[to_object_i].lt.y;
                    }

                    let mut multip: u32 = 1;
                    for (i, vector) in all_to_object_links_vec.iter_mut().enumerate() {
                        if i == to_object_i {
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
                    x_end += link_ends_stepsize * multip;
                    let mut xy2: XY = XY {
                        x: x_end,
                        y: y_end
                    };

                    let xy1y = xy1.y;

                    let link_layout: LineLayout = LineLayout {
                        start: xy1,
                        end: xy2,
                        base_first: base_line_first_half,
                        gap_first: link_gap_first,
                        gap_second: link_gap_second
                    };

                    link_layout_vec.push(link_layout);

                    if xy1y == base_line_first_half {
                        link_gap_first += ::REL_GAP_DISTANCE;
                    } else {
                        link_gap_second += ::REL_GAP_DISTANCE;
                    }
                }
            }
        }
    }

    (object_layout_vec, link_layout_vec, greatest_last_left_distance, top_line_second_half + greatest_height_second_half + 50)
}