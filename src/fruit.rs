use fastrand::Rng;

use crate::{palette, point::Point, wasm4};

pub struct Fruit;

impl Fruit {
    pub fn generate_position(
        snake_body: &Vec<Point>,
        grid_width: i32,
        grid_height: i32,
        rng: &mut Rng,
    ) -> Point {
        loop {
            let candidate = Point {
                x: rng.i32(0..grid_width),
                y: rng.i32(0..grid_height),
            };

            if !snake_body.contains(&candidate) {
                return candidate;
            }
        }
    }

    pub fn draw(position: Point, grid_segment_size: i32) {
        palette::set_draw_color(palette::Color::Fruit);
        wasm4::rect(
            position.x * grid_segment_size,
            position.y * grid_segment_size,
            grid_segment_size.try_into().unwrap(),
            grid_segment_size.try_into().unwrap(),
        );
    }
}
