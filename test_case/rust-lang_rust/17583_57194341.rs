 rust
// Thread-local static, not too useful because it can't be modified
tls!(static FOO: uint = 3);

// Thread-local unsafe cell, useful because it can be modified!
tls!(static FOO: UnsafeCell<uint> = ...);

// mutable, but still unsafe because borrows can be sent across threads
tls!(static FOO: uint = 3);
