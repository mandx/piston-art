extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

use piston_window::*;
use rand::prelude::*;
use sdl2_window::Sdl2Window;

fn draw_rect(
    slot_x: i32,
    slot_y: i32,
    factor: f64,
    radius: f64,
    context: &piston_window::Context,
    graphics: &mut G2d,
) {
    let rect_width = radius * 2.0;
    let start_x = f64::from(slot_x) * rect_width;
    let start_y = f64::from(slot_y) * rect_width;
    // let color = [random(), random(), random(), 1.0];
    let color = [0.0, 0.0, 0.0, 0.75];
    let transform = context
        .transform
        .trans(start_x + radius, start_y + radius)
        .rot_rad(factor);

    line(
        color,
        1.0,
        [-radius, -radius, radius, -radius],
        transform,
        graphics,
    );
    line(
        color,
        1.0,
        [radius, -radius, radius, radius],
        transform,
        graphics,
    );
    line(
        color,
        1.0,
        [radius, radius, -radius, radius],
        transform,
        graphics,
    );
    line(
        color,
        1.0,
        [-radius, radius, -radius, -radius],
        transform,
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

            let blocks_count_x: i32 = 6;
            let blocks_count_y: i32 = viewport_height / blocks_count_x;
            let radius: f64 = f64::from(viewport_width) / (f64::from(blocks_count_x) * 2.0);

            for slot_x in 0..blocks_count_x {
                for slot_y in 0..blocks_count_y {
                    draw_rect(
                        slot_x,
                        slot_y,
                        random::<f64>()
                            * (
                                1.0
                                /
                                (1.0 + f64::from(slot_y))
                            )
                            * (if random() { 1.0 } else { -1.0 }), // factor
                        radius,
                        &context,
                        graphics,
                    )
                }
            }
        });
    }
}
