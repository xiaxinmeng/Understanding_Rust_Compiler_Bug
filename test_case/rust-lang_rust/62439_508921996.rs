rust
self.boundaries
    .into_iter()
    .filter(|(x, y)| {
        self.points
            .keys()
            .map(|point| point.distance_to(*x, *y))
            .sum::<i32>()
            < max_distance
    })
    .count()
