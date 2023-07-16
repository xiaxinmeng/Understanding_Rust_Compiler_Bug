rust
use std::cell::Cell;
use cell_project::cell_project as cp; // renamed for ergonomics

struct Point {
    x: f32,
    y: f32,
}

fn get_x_cell(point: &Cell<Point>) -> &Cell<f32> {
    cp!(Point, point.x)
}
