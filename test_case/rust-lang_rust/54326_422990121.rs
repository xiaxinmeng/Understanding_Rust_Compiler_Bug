rust
fn foo() -> impl Fn() -> i32 {
    || {
        if false {
            return Ok(6);
        }
    
        5
    }
}
