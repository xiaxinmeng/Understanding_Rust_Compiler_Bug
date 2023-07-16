 rust
// this is a compile error
let slot = foo.borrow().get();

// this is valid code
match foo.borrow().get() {
    slot => { /* ... */ }
}
