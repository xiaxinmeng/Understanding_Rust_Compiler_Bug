rust
const LARGE_OBJECT: [T: Copy; N] = ...

// Warning, LARGE_OBJECT is copied to the stack and then indexed, this may lead to stack overflows.
// Perhaps LARGE_OBJECT should be a static instead?
LARGE_OBJECT[42]
