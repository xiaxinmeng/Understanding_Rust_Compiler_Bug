 Rust
struct Foo {f: uint, g: [uint * X]}
const X: uint = Z.f;
const Z: Foo = Foo { f: 3, g: [3, 2, 1] };
---
error: expected constant expr for vector length: Unsupported constant expr
struct Foo {f: uint, g: [uint * X]}
                        ^~~~~~~~~~
