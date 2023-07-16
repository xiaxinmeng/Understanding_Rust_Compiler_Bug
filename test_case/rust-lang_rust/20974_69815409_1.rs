 Rust
impl<C: Cache> Handler for fn(Context<C>, Response) {
    type Cache = C;

    fn handle_request(&self, context: Context<C>, response: Response) {
        (*self)(context, response);
    }
}
