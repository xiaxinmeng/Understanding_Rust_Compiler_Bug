
use std::collections::BTreeMap;
use std::sync::Arc;
fn foo<T: Send>() { }
fn main() {
    foo::<BTreeMap<Arc<()>, Arc<()>>>();
}
