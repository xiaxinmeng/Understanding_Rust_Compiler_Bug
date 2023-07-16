 rust
default fn print<T>(t: &T) { ... }
default fn print<T: Debug>(t: &T) { ... }
default fn print<T: Display>(t: &T) { ... }
fn print<T: Debug + Display>(t: &T) { ... }
