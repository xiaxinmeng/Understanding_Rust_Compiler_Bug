rust
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {{
        #[deny(unreachable_patterns)]
        let tmp = match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        };
        tmp
    }}
}
