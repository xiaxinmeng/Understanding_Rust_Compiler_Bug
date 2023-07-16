rust
fn f(x: &Option<i32>, y: &Option<i32>) -> i32 {
    if let &Some(_) = x {
        if let &Some(_) = y {
            return 1
        }
    }
    42
}

f(&None, /* invalid ref obtained through unsafe? */)
