rust
const fn foo() {}
let x: const fn() = foo;
let y: fn() = x; // OK: const fn => fn coercion
