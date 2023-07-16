

One effect of this definition is that `var_os` no longer satisfies the type `for<'r> fn(&'r K)` but rather the type `fn(&'r K)`. The difference is subtle -- the first type says that you can call this function pointer multiple times with distinct lifetimes each time, the second says that the function pointer is associated with a single lifetime for each reference. In other words, if you write multiple calls to `var_os()`, you'll never notice the difference, but if you capture a single reference to `var_os`, you might:

