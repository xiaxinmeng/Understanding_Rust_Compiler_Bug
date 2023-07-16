Rust
trait MyTrait: FnMut() -> <Self as MyTrait>::MyOutput {
    type MyOutput;
}
