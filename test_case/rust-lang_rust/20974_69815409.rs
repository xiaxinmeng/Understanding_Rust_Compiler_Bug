 Rust
impl<C: Cache, F: Fn(Context<C>, Response)> Handler for F {
    type Cache = C;

    fn handle_request(&self, context: Context<C>, response: Response) {
        (*self)(context, response);
    }
}
