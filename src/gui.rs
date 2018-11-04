// the gui module has its own namespace, as well as "extern crate" from main has.
// when calling from conrod, you have to import conrod again
use conrod::{self};
// this is standard in conrod
use conrod::backend::glium::glium::{self, Surface};

pub fn start() {

    // these are default values

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    'main: loop {
        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}