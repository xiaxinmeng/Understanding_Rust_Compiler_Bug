 rust
#[deriving(Clone)]
struct Map<'a, A, B, I> {
    iter: I,
    f: |A|:'a -> B,  //~ error: type `|A| -> B` does not implement any method in scope named `clone`
}
