rust
pub fn foo(num: i32) -> i32 {
    match num {
        100..=199 => num / 100,
        _ => 1,
    }
}
