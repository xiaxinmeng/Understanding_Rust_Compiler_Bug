rust
fn main() {
    mod foo {
        use super::*;
        
        fn foobar() {
            bar();
        }
    }
}

fn bar() {}
