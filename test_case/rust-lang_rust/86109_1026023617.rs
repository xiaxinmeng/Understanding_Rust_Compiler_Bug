rust
pub fn foo(start: T) -> T {
    if start <= 0 { return 0; }
    let start = start as u16; // add this to change type
    let mut count = 0;
    ...
