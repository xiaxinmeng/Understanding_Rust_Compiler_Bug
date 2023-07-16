
struct foo { x: i32, y: i32 }
static A : foo = foo { x: 3, y: 4 };

#[cfg(field_proj)]
static B : i32 = A.x;

#[cfg(inline_val)]
static B : i32 = 3;

fn main() {
    let v : [i32; B] = [5, 6, 7];
    println!("{:?}", v);
}
