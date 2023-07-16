
File: src/librustc/mir/mod.rs
622:50:    /// Some((None, ..)) in the case of and `let [mut] x = ...` because
File: src/librustc_mir/transform/add_retag.rs
174:61:                    // to update tags.  This includes `x = &[mut] ...` and hence
File: src/librustc_mir/borrow_check/move_errors.rs
127:56:                    // opt_match_place is None for let [mut] x = ... statements,
File: src/libcore/pin.rs
163:71://! in your type (for example by implementing some operation on `Pin<&[mut] Self>`)
176:37://! an operation with type `fn(Pin<&[mut] Struct>) -> Pin<&[mut] Field>`?
178:42://! have an operation with type `fn(Pin<&[mut] Wrapper<T>>) -> Pin<&[mut] T>`?
182:59://! After all, the pinning projection lets us get a `Pin<&[mut] Field>`.
