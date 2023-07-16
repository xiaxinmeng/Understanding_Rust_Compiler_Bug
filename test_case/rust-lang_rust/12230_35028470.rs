 rust
fn by_ref<'a>(&'a mut self) -> ByRefReader<'a> { ... }
fn by_ref<'a>(&'a mut self) -> ByRefWriter<'a> { ... }
