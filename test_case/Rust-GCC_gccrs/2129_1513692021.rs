rust
macro_rules! matches {
    ($( $pattern:pat )|+ $( if $guard:expr ),*) => {
        match b'A' {
            $( $pattern )|+ => {}
        }
    }
}

pub fn foo() {
    matches!(b'A')
}
