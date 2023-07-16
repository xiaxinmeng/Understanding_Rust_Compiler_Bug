 rust
enum Foo { // "warning: mismatched variant sizes"
   A,
   B(int, int, int, int, int, int, int, int) // "note: this variant is much larger than the others"
}

// could become

struct BInner(int, int, int, int, int, int, int, int);
enum Foo {
   A,
   B(~BInner)
}
