 rust
let x: [i32; 3] = [1, 2, 3];
match x {
    [1, b..] => { /* b is [i32; 2] */ }
    [2, ref b..] => { /* b is &[i32; 2] */ }
    [a, ref mut b..] => { /* b is &mut [i32; 2] */ }
}

let x: &mut [i32] = &mut [1, 2, 3];
match x {
    // &mut [1, b..] => { /* b is [i32] (invalid because b is !Sized) */ }
    &mut [2, ref b..] => { /* b is &[i32] */ }
    &mut [3, ref mut b..] => { /* b is &mut [i32] */ }
    _ => {}
}
