 rust
// A `type` which forms a total order is completely sortable
// `i32` forms a Total Order
let _entirelySortable = [1i32, -1, 7, 3, 5, 9]; // Sortable because `i32` is

// `f32` does not form a Total Order
let _partiallySortable1 = [1.0f32, -1.0, 7.0, std::f32::NAN];
// NAN's sorting order is undefined making this `type` unsortable
// (Is NAN before or after 3?)

// Still unsortable because `f32` supports NAN
let _partiallySortable2 = [1.0f32, -1.0, 7.0, 14.2]; // Also, not a total order
