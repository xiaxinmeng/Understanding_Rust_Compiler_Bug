rust
fn caller<F>(f: F)  where F: Fn(&i32) {
    f(&42);
}

fn caller_ref_return<F>(f: F) where F: Fn(&i32) -> &i32 {
    f(&42);
}

fn main() {
    // Works
    caller(|a| println!("{}", a));
    
    // Works 
    let f = |a: &i32| println!("{}", a);
    caller(f);
    
    // Doesn't work (error: type mismatch)
    let f = |a| println!("{}", a);
    caller(f);
    
    // Works 
    caller_ref_return(|a| a);
    
    // Doesn't work (error: expected bound lifetime parameter, found concrete lifetime)
    let f = |a: &i32| a;
    caller_ref_return(f);
    
    // Doesn't work (error: type mismatch)
    let f = |a| a;
    caller_ref_return(f);
}
