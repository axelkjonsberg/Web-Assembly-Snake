use crate::{palette, point::Point, wasm4};
use std::collections::HashSet;
use std::hash::Hash;

pub struct Snake;

impl Snake {
    pub fn draw(body: &Vec<Point>, grid_segment_size: i32) {
        for point in body.iter().skip(1) {
            Snake::draw_segment(
                point.x,
                point.y,
                palette::Color::SnakeBody,
                grid_segment_size,
            );
        }

        if let Some(head) = body.first() {
            Snake::draw_segment(head.x, head.y, palette::Color::SnakeHead, grid_segment_size);
        }
    }

    fn draw_segment(x: i32, y: i32, color: palette::Color, grid_segment_size: i32) {
        palette::set_draw_color(color);
        wasm4::rect(
            x * grid_segment_size,
            y * grid_segment_size,
            grid_segment_size.try_into().unwrap(),
            grid_segment_size.try_into().unwrap(),
        );
    }

    pub fn move_body(
        body: &mut Vec<Point>,
        current_direction: Point,
        grid_width: i32,
        grid_height: i32,
    ) {
        let new_head = Point {
            x: (body.first().unwrap().x + current_direction.x + grid_width) % grid_width,
            y: (body.first().unwrap().y + current_direction.y + grid_height) % grid_height,
        };

        body.insert(0, new_head);
        body.pop();
    }

    pub fn change_direction(current_direction: &mut Point, new_direction: Point) {
        if current_direction.x + new_direction.x != 0 || current_direction.y + new_direction.y != 0
        {
            *current_direction = new_direction;
        }
    }

    pub fn up(direction: &mut Point) {
        Snake::change_direction(direction, Point { x: 0, y: -1 });
    }

    pub fn down(direction: &mut Point) {
        Snake::change_direction(direction, Point { x: 0, y: 1 });
    }

    pub fn left(direction: &mut Point) {
        Snake::change_direction(direction, Point { x: -1, y: 0 });
    }

    pub fn right(direction: &mut Point) {
        Snake::change_direction(direction, Point { x: 1, y: 0 });
    }

    pub fn grow(body: &mut Vec<Point>, direction: Point, grid_width: i32, grid_height: i32) {
        let new_head = {
            let head = body.first().expect("Snake has no body");
            Point {
                x: (head.x + direction.x + grid_width) % grid_width,
                y: (head.y + direction.y + grid_height) % grid_height,
            }
        };
        body.insert(0, new_head);
    }

    pub fn is_overlapping_itself<T: Eq + Hash>(body: &[T]) -> bool {
        let mut unique_segments = HashSet::with_capacity(body.len());

        for segment in body {
            if unique_segments.contains(segment) {
                return true;
            }
            unique_segments.insert(segment);
        }

        false
    }
}
