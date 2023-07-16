rust
macro_rules! foo {
    () => {{
        let x = 1 + 1;
        println!("{}", x);
    }};
}

fn main() {
    foo!(); // #break
}
