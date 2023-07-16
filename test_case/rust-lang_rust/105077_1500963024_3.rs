rust
#[restrict(override(crate), call(crate))]
fn type_id(&self) -> TypeId where Self: 'static
