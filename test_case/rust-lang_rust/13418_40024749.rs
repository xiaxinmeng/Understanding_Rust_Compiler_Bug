 rust
fn c() {
    let mut vec = vec!(~1, ~2, ~3);
    let vec: &mut [~int] = vec.as_mut_slice();
    match vec {
        [_a, ..b] => {}
        _ => {}
    }
}

test.rs:5:9: 5:18 error: cannot move out of dereference of `&mut`-pointer
test.rs:5         [_a, ..b] => {}
                  ^~~~~~~~~
test.rs:5:10: 5:12 note: attempting to move value to here (to prevent the move, you can use `ref _a` to capture value by reference)
test.rs:5         [_a, ..b] => {}
                   ^~
