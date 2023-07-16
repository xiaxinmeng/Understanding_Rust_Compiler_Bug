 rust
macro_rules! matches(
    ($e: expr, $($p :pat)|*) => (match $e {
        $($p)|+ => true,
        _ => false
    });
    ($e: expr, $($p :pat)|* if $g: expr) => (match $e {
        $($p)|+ if $g => true,
        _ => false
    });
)
