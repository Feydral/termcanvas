mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{
    canvas::{Canvas, draw::Align, font::Font},
    input::Input,
    math::{mathi, noise},
};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let mut last_frame = std::time::Instant::now();
    let mut display_fps = 0.5;
    let mut fps = 0;

    loop {
        let _ = input.update();
        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        let now = std::time::Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        display_fps += dt;

        let new_w = Canvas::terminal_width();
        let new_h = Canvas::terminal_height();
        if new_w as u32 != canvas.width() || new_h as u32 != canvas.height() {
            canvas.resize(new_w, new_h);
        }

        if display_fps > 1.0 {
            display_fps = 0.0;
            fps = (1.0 / dt) as u32;
        }

        canvas.clear();

        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                let scale = 0.005;
                let warp_scale = 0.008;
                let warp_strenght = 80.0;

                let wx = x as f32
                    + noise::get_simplex_2d(0, x as f32 * warp_scale, y as f32 * warp_scale)
                        * warp_strenght;
                let wy = y as f32
                    + noise::get_simplex_2d(1, x as f32 * warp_scale, y as f32 * warp_scale)
                        * warp_strenght;

                let color =
                    (1.0 - noise::get_smooth_simplex_2d(2, wx * scale, wy * scale).abs()) * 255.0;

                canvas.set_pixel(
                    x,
                    y,
                    mathi::rgb_to_u32(color as u8, color as u8, color as u8),
                );
            }
        }

        let blue = mathi::rgb_to_u32(127, 127, 255);
        let red = mathi::rgb_to_u32(255, 127, 127);

        canvas
            .draw(&font)
            .at(5, 5)
            .color(red)
            .align(Align::Left)
            .text("Warped Simplex Noise Test");

        let x = canvas.width().saturating_sub(5);
        let y = canvas.height().saturating_sub(13);

        canvas
            .draw(&font)
            .at(x, y)
            .color(blue)
            .align(Align::Right)
            .uint(fps);

        canvas.render();
    }
}