 rust
struct C;

trait Num {
    type Float: Copy;
}

extern fn csrot_wrap(_: <C as Num>::Float) { }

impl Num for C {
    type Float = f32;
