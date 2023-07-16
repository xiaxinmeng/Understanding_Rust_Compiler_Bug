rs
#[derive(Debug)]
struct A {
    one: i32,
    two: i32,
    three: i32,
}

impl Default for A {
    fn default() -> Self {
        A {
            one: 1,
            two: 2,
            three: 3,
        }
    }
}

dbg!(A {
    one: 111,
    ..default() // expecting `two: 2`, not `two: 0`
});
