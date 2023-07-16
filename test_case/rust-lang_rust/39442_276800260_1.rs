
macro_rules! derive {
    () => { println!("HI!"); }
}

// Doesn't work today, except if this appears before the macro_rules!.
#[derive(Default)]
struct A;

fn main() {
    derive!();
}
