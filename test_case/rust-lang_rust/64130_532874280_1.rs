rust
async fn foo() {...}
async fn bar() {...}
fn is_send<T: Send>(_: T) { }
fn main() {
    is_send((foo(), bar())); // introduces a tuple type into the chain
}
