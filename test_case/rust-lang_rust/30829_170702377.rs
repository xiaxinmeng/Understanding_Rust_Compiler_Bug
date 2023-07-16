 Rust
#![feature(unboxed_closures, fn_traits)]

fn test2<R>(r: &mut R) {
    let _f = |r: &mut R| {
        let x = || -> () { let _ = &r; };
        Fn::call(&x, ()); // compiles
        // x(); //~ ERROR the parameter type `R` may not live long enough [E0311]
    };
}

fn main() { 
}
