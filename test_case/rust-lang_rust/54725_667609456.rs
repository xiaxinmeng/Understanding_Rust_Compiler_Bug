rust
// main.rs
mod submod {
    macro_rules! make_local_weird {
        ($local:item) => (
            #[macro_export]
            macro_rules! weird {
                () => (
                    println!("{}", export! { $local private })
                )
            }

            pub use weird;
        )
    }

    make_local_weird!(fn f() {});
}

pub fn main() {
    let private = "hi";
    submod::weird!();
}
