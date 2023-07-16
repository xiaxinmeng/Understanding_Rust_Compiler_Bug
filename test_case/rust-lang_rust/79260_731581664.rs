rust
fn main() {
    mod foo {
        use super::*;
        
        fn foobar() {
            bar();
        }
    }

    fn bar() {
        println!("inner bar");
    }
}

fn bar() {
    println!("outer bar");
}
