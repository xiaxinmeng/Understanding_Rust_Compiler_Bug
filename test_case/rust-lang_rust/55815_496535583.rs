rust
fn boxed(f: Box<dyn FnOnce() -> i32>) -> i32 {
    f()
}

assert_eq!(boxed(Box::new({let x = 13; move || x})), 13);
