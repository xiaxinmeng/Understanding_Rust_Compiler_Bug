
The only function calls allowed in static or constant expressions are
constructors for enum variants and structs, or `const fn`s. Rust currently does
not support more general compile-time function execution.

See [RFC 911](https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn
.md) for more details on the design of `const fn`s.
