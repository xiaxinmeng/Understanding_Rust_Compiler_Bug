 Rust
trait Array: Deref<Target = [<Self as Array>::Item]> {
    type Item;
}
