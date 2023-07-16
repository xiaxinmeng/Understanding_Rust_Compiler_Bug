 rust
let mut v = [1, 2, 3];
match v {
    [ref mut x, ref mut y, ref mut z] => {
        *x += 1;
        *y += 1;
        *z += 1;
    }
}
