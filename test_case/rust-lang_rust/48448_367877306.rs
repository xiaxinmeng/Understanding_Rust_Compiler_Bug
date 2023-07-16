rust
let (x, &(y, ref z)) = &(1, (2, 3));
// x: &i32
// y: i32
// z: &i32

let (x, &Some(Some(y))) = &(1, Some(Some(5)));
// x: &i32
// y: i32
