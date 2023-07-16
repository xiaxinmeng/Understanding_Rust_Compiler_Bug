rust
#![feature(unsized_fn_params)]

pub fn f(k: dyn std::fmt::Display) {
    let k2 = Box::new(move || { let x = k.to_string(); });
}
