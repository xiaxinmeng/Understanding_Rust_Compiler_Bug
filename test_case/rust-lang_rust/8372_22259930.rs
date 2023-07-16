
// Work around "error: cannot borrow `iter` as mutable more than once at a time"
// when using a normal for loop.
macro_rules! for_iter(
    ($iter: ident, $pattern: pat, $loop_body: expr) => (
        loop {
            match $iter.next() { None => break, Some($pattern) => $loop_body }
        }
    );
)
