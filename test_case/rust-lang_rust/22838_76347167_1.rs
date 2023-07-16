
let a: &[_] = [1, 2, 3];
const A: &'static [i32; 3] = &[2, 3, 4];
match a {
    A => ...
}
