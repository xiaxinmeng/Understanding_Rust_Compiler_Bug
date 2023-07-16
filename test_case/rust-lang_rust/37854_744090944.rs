rust
// 2. For each coord in arrow_coords:
//
//   A. Call `coord.print_description()`
//   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
//   `coord.distance_from_center()`
//      - Less than 1.0 -- `Shot::Bullseye`
//      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
//      - Greater than 5.0 -- `Shot::Miss`
for coord in arrow_coords {
    coord.print_description();
    let dist = coord.distance_from_center();
    shots.push(match dist {
        0.0..1.0 => Shot::Bullseye,
        1.0..5.0 => Shot::Hit(dist),
        _ => Shot::Miss,
    });
}
