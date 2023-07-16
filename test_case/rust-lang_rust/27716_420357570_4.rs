rust
pub fn with_context<F, R>(f: F) -> R
where
    F: FnOnce(&Arc<Context>) -> R,
{
    CONTEXT.try_with(|cx| {
        match cx {
            Ok(cx) => f(cx),
            Err(..) => f(&Context::new()),
        }
    })
}
