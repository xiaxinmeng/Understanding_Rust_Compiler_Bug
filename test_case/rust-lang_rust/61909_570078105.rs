rust
// Grid is a thin wrapper around a HashMap<(i64, i64), Cell> representing a 2D grid of values.
// Cell here is just an enum, not std::cell::Cell.
fn get_intersections<'a>(grid: &'a Grid<Cell>) -> impl Iterator<Item = &'a Coord> {
    grid.iter()
        // Filter to nonempty cells.
        .filter_map(|(coord, cell)| {
            if *cell != Cell::Empty {
                Some(coord)
            } else {
                None
            }
        })
        // Intersections are nonempty locations with at least 3 nonempty neighbors.
        .filter(|coord| {  // This needs to be a `move` closure to capture `grid`.
            let num_adjacent = grid
                .neighbors(coord)
                .filter(|&(_, cell)| cell != Cell::Empty)
                .count();
            num_adjacent >= 3
        })
}
