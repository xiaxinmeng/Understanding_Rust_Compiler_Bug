 rust
struct foo { x: usize, y: usize }
const A : foo = foo { x: 3, y: 4 };

#[cfg(field_proj)]
const B : usize = A.x;

#[cfg(inline_val)]
const B : usize = 3;

fn main() {
    let v : [i32; B] = [5, 6, 7];
    println!("{:?}", v);
}
