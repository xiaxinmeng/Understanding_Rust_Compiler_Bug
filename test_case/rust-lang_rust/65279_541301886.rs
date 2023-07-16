rust
struct Struct<T> {
    s: T,
    t: T,
}

let x = &mut 0i32;
let y = &mut 1i32;

let u = Struct{ s: x, t: y }; // x is moved, y is reborrowed
let v = Struct{ t: y, s: x }; // y is moved, x is reborrowed
