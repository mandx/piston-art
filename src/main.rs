extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

use piston_window::*;
use rand::prelude::*;
use sdl2_window::Sdl2Window;

fn draw_rect(
    slot_x: usize,
    slot_y: usize,
    factor: f64,
    radius: f64,
    context: &piston_window::Context,
    graphics: &mut G2d,
) {
    let start_x = (slot_x as f64) * radius * 2.0;
    let start_y = (slot_y as f64) * radius * 2.0;
    let end_x = start_x + radius * 2.0;
    let end_y = start_y + radius * 2.0;

    rectangle(
        [random(), random(), random(), 1.0], // red
        [start_x, start_y, end_x, end_y],    // rectangle
        context
            .transform
            // .clone()
            // .trans(start_x + radius, start_y + radius)
            .rot_rad(factor),
        graphics,
    );
}

fn main() {
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .decorated(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics: &mut G2d| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            let (viewport_width, viewport_height) = context
                .viewport
                .map(|v| (v.rect[2], v.rect[3]))
                .unwrap_or((640, 480));

            let blocks_count_x: usize = 6;
            let blocks_count_y: usize = 6;
            let radius: f64 = (viewport_width as f64) / ((blocks_count_x as f64) * 2.0);

            for slot_x in 0..blocks_count_x {
                for slot_y in 0..blocks_count_y {
                    draw_rect(
                        slot_x,
                        slot_y,
                        random(), // factor
                        radius,
                        &context,
                        graphics,
                    )
                }
            }
        });
    }
}
