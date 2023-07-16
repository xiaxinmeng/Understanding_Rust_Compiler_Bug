rust
for (x, y) in &self.boundaries {
    let mut min_distance = i32::max_value();
    let mut closest = None;

    for (point, area) in self.points.iter_mut() {
        ...
    }
    ...
