rust
pub fn alt_fn<'lua>(mut fut: Box<dyn 'static + FnMut(&'lua())>) {
    take_closure(fut);
}
