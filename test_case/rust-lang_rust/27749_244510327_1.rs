 rust
fn print<T>(t: &T) {
    if T::implements::<Debug>() { ... }
    else if T::implements::<Display>() { ... }
    else { ... }
} 
