use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;

use piet::kurbo::{Circle, Rect, Size};
use piet::{Color, FontBuilder, RenderContext, Text, TextLayoutBuilder};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut cg_ctx = CGContext::create_bitmap_context(
        None,
        WIDTH,
        HEIGHT,
        8,
        0,
        &CGColorSpace::create_device_rgb(),
        core_graphics::base::kCGImageAlphaPremultipliedLast,
    );
    let mut piet = piet_coregraphics::CoreGraphicsContext::new(&mut cg_ctx);
    let bounds = Size::new(WIDTH as f64, HEIGHT as f64).to_rect();
    piet.stroke(bounds, &Color::rgba8(0, 255, 0, 128), 20.0);
    piet.fill(
        bounds.inset((0., 0., -bounds.width() * 0.5, 0.)),
        &Color::rgba8(0, 0, 255, 128),
    );
    piet.fill(
        Circle::new((100.0, 100.0), 50.0),
        &Color::rgb8(255, 0, 0).with_alpha(0.5),
    );
    piet.fill(Rect::new(0., 0., 200., 200.), &Color::rgba8(0, 0, 255, 128));

    let font = piet
        .text()
        .new_font_by_name("Georgia", 24.0)
        .build()
        .unwrap();

    let layout = piet
        .text()
        .new_text_layout(&font, "this is my cool\nmultiline string, I like it very much, do you also like it? why or why not? Show your work.", 400.0)
        .build()
        .unwrap();

    piet.draw_text(&layout, (200.0, 200.0), &Color::BLACK);
    piet.draw_text(&layout, (0., 00.0), &Color::WHITE);
    piet.draw_text(&layout, (400.0, 400.0), &Color::rgba8(255, 0, 0, 150));

    piet.finish().unwrap();

    unpremultiply(cg_ctx.data());

    // Write image as PNG file.
    let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(cg_ctx.data()).unwrap();
}

fn unpremultiply(data: &mut [u8]) {
    for i in (0..data.len()).step_by(4) {
        let a = data[i + 3];
        if a != 0 {
            let scale = 255.0 / (a as f64);
            data[i] = (scale * (data[i] as f64)).round() as u8;
            data[i + 1] = (scale * (data[i + 1] as f64)).round() as u8;
            data[i + 2] = (scale * (data[i + 2] as f64)).round() as u8;
        }
    }
}