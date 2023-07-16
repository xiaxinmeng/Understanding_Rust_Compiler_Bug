rust
pub fn foo(a: &mut u8, q: i32) -> i32 {
    *a = if q == 5 {
        1
    } else {
        2
    };
    match *a {
        1 => 1,
        2 => 2,
        _ => unreachable!(),
    }
}
