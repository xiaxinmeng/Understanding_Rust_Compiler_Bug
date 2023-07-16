rust
// Point is struct { x: i32, y: i32 } with derived Hash
// self.points is HashMap<Point, i32>
let mut points = self.points.iter_mut().collect::<Vec<_>>();

for (x, y) in &self.boundaries {
    let mut min_distance = i32::max_value();
    let mut closest = None;

    for (point, area) in points.iter_mut() {
        ...
    }
    ...
