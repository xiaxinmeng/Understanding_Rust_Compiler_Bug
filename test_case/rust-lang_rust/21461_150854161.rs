 rust
#![feature(core, unboxed_closures)]

struct VecEnv<'a> {
    vec: Vec<i32>,
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, 'b, 'c> FnOnce<(&'b mut i32, &'c i32)> for VecEnv<'a> {
    type Output = i32;

    extern "rust-call" fn call_once(self, (output, text): (&'b mut i32, &'c i32)) -> i32 {
        println!("CLOSURE");
        1i32
    }
}

fn main() {}
