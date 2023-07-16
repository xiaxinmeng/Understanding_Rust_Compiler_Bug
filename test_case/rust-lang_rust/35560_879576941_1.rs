Rust
fn main() {
    macro_rules! my_macro {
        () => {
            1,2,3 //~ERROR macro expansion ignores token `,` and any following
        }
    }

    let v = [my_macro!()];
    println!("{:?}", v);
}
