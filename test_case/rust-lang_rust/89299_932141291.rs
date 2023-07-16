rust
fn ref_id<T /* : Sized */>(it: &T) -> &T { it }
let _: &dyn Sync = ref_id(&()); // Error, `dyn Sync` is not `Sized`
