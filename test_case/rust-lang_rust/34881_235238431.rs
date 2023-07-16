 rust
macro_rules! emit_bug {
// if nothing on input, then return what?
    () => (); // () => (()); <-- This works
}

fn main() {
   emit_bug!() // no semicolon here
}
