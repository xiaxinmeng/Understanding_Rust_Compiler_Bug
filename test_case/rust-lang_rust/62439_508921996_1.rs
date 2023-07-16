rust
// Point is struct { x: i32, y: i32 } with derived Hash
// self.points is HashMap<Point, i32>
let points = self.points.keys().collect::<Vec<_>>();

self.boundaries
    .into_iter()
    .filter(|(x, y)| {
        points
            .iter()
            .map(|point| point.distance_to(*x, *y))
            .sum::<i32>()
            < max_distance
    })
    .count()
