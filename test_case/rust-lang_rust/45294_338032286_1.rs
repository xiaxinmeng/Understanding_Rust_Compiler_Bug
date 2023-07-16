rust
type A = &A + B; // ERROR
type B = &dyn A + B; // OK
