
macro_rules! foo(
    ($var:ident, $code:expr) => (
        if test($var) {
            let var = cond.raise($var.to_owned());
            $code
        } else {
            $code
        }
    )
)
