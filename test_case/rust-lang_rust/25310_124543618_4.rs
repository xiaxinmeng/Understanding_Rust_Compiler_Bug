

In this case, the code was capturing a single reference to `var_os` and using it as a closure. Converting the code to use explicit lifetimes changed the closure so that it only accepts references with the lifetime `'a`, versus any lifetime. Another fix would be to change from `foo(&env::var_os)` to `foo(|x| env::var_os(x))`. 

I'm going to close this, since the code has made it to stable and doesn't seem to have caused dramatic impact, but it's an interesting regression vector.

cc @rust-lang/libs <-- this should be kept in mind in the future.
