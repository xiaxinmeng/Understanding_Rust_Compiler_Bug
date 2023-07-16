 rust
struct Map<B,I,F>
    where I : Iterator, F : FnMut(I::Item) -> B
...
