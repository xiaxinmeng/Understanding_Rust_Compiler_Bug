rust
pub fn with_context<F, R>(f: F) -> R
where
    F: FnOnce(&Arc<Context>) -> R,
{
    CONTEXT.try_with(|cx| f(cx)).unwrap_or_else(|_| f(&Context::new()))
}
