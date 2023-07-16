plain
   Compiling ignore v0.4.17
   Compiling merge_derive v0.1.0
   Compiling merge v0.1.0
   Compiling toml v0.5.7
error[E0599]: no method named `startswith` found for struct `Interned<std::string::String>` in the current scope
    --> src/bootstrap/builder.rs:1240:58
     |
1240 |         if !mode.must_support_dlopen() && !target.triple.startswith("powerpc-") {
     |                                                          ^^^^^^^^^^ help: there is an associated function with a similar name: `starts_with`
    ::: src/bootstrap/cache.rs:20:1
     |
     |
20   | pub struct Interned<T>(usize, PhantomData<*const T>);
     | ----------------------------------------------------- method `startswith` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `bootstrap`
