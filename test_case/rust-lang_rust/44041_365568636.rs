rust
pub fn mytest(x: i32, y: i32, i: usize) -> usize {
    let x = if x == y { ::std::process::exit(0) }
    else if x < y { 0 }
    else { 1 };
    
    2 * (i + 1) - x
}
