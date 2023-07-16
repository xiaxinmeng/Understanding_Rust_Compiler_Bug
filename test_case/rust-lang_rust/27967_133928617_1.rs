 rust
macro_rules! foo {
    ($key:expr) => {
        match "foo" {
            $key => "",
            _ => "",
        }
    }
}

fn main() {
    foo! {
        "foo"
    };
}
