rust
// Name `S` is still defined, but referring to it is an error if the where predicate ends up being false during type checking, similarly to false bounds with `#![feature(trivial_bounds)]`.
struct S where exists(Type::dependent) {
    ...
}
