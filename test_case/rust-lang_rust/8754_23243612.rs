
macro_rules! foo(
    ($code:block) => (
        if true $code else $code
    )
)
