 rust
TypeId::of::<fn(&'static int)>() != TypeId::of::<for<'a> fn(&'a int)>()
