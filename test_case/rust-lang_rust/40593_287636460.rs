rust
#[deny(warnings)]

struct T{}

impl Drop for T {
        fn drop(&mut self) {
                //println!("dropped");
                panic!(false);
        }
}

fn main() {
    {
        Some(T{});
    }
}
