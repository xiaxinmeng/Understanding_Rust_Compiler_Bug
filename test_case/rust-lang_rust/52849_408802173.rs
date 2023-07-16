rust
static BIG_ARRAY_OF_TUPLES: [(i32, (f64, f64, f64)); 0xffff] = [
    (641161, (2.988, 10.544, 29.598)),
    ...
    (209150, (5.216, 15.229, 23.389)),
];

fn main() {
    assert!(BIG_ARRAY_OF_TUPLES.len() == 0xffff);
}
