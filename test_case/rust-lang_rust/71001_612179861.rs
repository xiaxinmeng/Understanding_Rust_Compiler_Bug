rust
// Link "static-nobundle" native libs only if the crate they originate from
// is being linked statically to the current crate.  If it's linked dynamically
// or is an rlib already included via some other dylib crate, the symbols from
// native libs will have already been included in that dylib.
