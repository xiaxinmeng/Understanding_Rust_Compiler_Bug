 rust
error: the requirement `I : marker::Sized` appears on the impl method but not on the corresponding trait method [E0276]
fn nth(&mut self, n: usize) -> Option<I::Item> where I: Sized { (**self).nth(n) }
