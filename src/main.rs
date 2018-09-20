extern crate find_folder;
extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

mod fps_counter;

use piston_window::*;
use rand::prelude::*;
use sdl2_window::Sdl2Window;

use fps_counter::FPSCounter;

static SQUARES_COLOR: &[f32; 4] = &[0.0, 0.0, 0.0, 0.5];
static TEXT_COLOR: &[f32; 4] = &[0.0, 0.0, 0.0, 1.0];

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

    let transform = context
        .transform
        .trans(start_x + radius, start_y + radius)
        .rot_rad(factor);

    line(
        *SQUARES_COLOR,
        1.0,
        [-radius, -radius, radius, -radius],
        transform,
        graphics,
    );
    line(
        *SQUARES_COLOR,
        1.0,
        [radius, -radius, radius, radius],
        transform,
        graphics,
    );
    line(
        *SQUARES_COLOR,
        1.0,
        [radius, radius, -radius, radius],
        transform,
        graphics,
    );
    line(
        *SQUARES_COLOR,
        1.0,
        [-radius, radius, -radius, -radius],
        transform,
        graphics,
    );

    // rectangle(*SQUARES_COLOR, [
    //     -radius, -radius, radius * 2.0, radius * 2.0,
    //     ], transform, graphics)
}

fn draw_fps(fps: usize, glyphs: &mut Glyphs, context: &piston_window::Context, graphics: &mut G2d) {
    let transform = context.transform.trans(20.0, 50.0);

    if let Err(error) = text::Text::new_color(*TEXT_COLOR, 32).draw(
        &format!("{:?}", fps),
        glyphs,
        &context.draw_state,
        transform,
        graphics,
    ) {
        println!("Text rendering error {:?}", error);
    }
}

fn main() {
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .decorated(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let font = &find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Failed to locate assets folder")
        .join("FiraSans-Regular.ttf");

    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut fps_counter = FPSCounter::new();
    let mut blocks_count_x = 9;

    while let Some(event) = window.next() {
        if let Some(delta) = event.mouse_scroll(|_x, y| y as i32) {
            blocks_count_x = match blocks_count_x + delta {
                n if n > 0 => n,
                _ => 1,
            };
        }

        let fps = fps_counter.tick();

        window.draw_2d(&event, |context, graphics: &mut G2d| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            let (viewport_width, viewport_height) = context
                .viewport
                .map_or((640, 480), |v| (v.rect[2], v.rect[3]));

            let blocks_count_y: i32 = viewport_height / ((viewport_width / blocks_count_x) + 1);
            let radius: f64 = f64::from(viewport_width) / (f64::from(blocks_count_x) * 2.0);

            for slot_x in 0..blocks_count_x {
                for slot_y in 0..blocks_count_y {
                    draw_rect(
                        slot_x,
                        slot_y,
                        random::<f64>()
                            * (f64::from(slot_y) / 50.0)
                            * (if random() { 1.0 } else { -1.0 }), // factor
                        radius,
                        &context,
                        graphics,
                    );

                    draw_fps(fps, &mut glyphs, &context, graphics);
                }
            }
        });
    }
}
