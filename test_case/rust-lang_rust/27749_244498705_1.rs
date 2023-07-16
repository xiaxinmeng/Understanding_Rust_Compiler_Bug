 rust
fn print<T>(t: &T) {
    if T::implements::<Display>() { ... }
    else if T::implements::<Debug>() { ... }
    else { ... }
} 
