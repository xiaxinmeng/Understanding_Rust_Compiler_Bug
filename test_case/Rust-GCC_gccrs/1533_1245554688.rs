rust
mod foo {
    #[macro_export]
    macro_rules! bar1 {
        () => {};
    }

    macro_rules! bar2 {
        () => {};
    }
}

fn main() {
    bar1!();
    bar2!(); // { dg-error "cannot find macro .bar2. in this scope" }
}
