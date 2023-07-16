
error[E0597]: `grid` does not live long enough
   --> src/problem17.rs:214:32
    |
202 | fn get_intersections<'a>(grid: &'a Grid<Cell>) -> impl Iterator<Item = &'a Coord> {
    |                      --                           ------------------------------- opaque type requires that `grid` is borrowed for `'a`
    |                      |
    |                      lifetime `'a` defined here
...
213 |         .filter(|coord| {
    |                 ------- value captured here
214 |             let num_adjacent = grid
    |                                ^^^^ borrowed value does not live long enough
...
220 | }
    | - `grid` dropped here while still borrowed
