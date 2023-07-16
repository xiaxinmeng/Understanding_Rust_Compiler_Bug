rust
macro_rules! callback {
    () => {
        println!("called.");
    };
}

fn main() {
    call_my_callback!();
}
