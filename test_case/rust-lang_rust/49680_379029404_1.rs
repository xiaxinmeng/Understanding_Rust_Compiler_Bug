rust
struct Array<U, const N> { ... }

type A = Array<U, N>; // OK without braces if N resolves to a value
