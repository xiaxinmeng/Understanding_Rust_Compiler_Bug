rust
fn last_big(vec: &mut Vec<i32>) -> &mut i32 {
    match **vec {
        [.., ref mut last] if *last > 99 => last,
        _ => {
            vec.push(100); // ERROR: can't borrow mutably again
            vec.last_mut().unwrap()
        }
    }
}
