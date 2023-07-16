
fn a<F: for<'s> Fn() -> Vec<&'s i32>>(f: F) 
