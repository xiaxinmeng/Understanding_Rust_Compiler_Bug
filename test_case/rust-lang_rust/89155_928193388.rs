plain
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.99
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
error: expected one of `extern` or `fn`, found keyword `const`
    |
277 | impl UnsafeArg {
    |                - while parsing this item list starting here
...
...
283 |     pub unsafe const fn new() -> Self {
    |         |      |
    |         |      |
    |         |      expected one of `extern` or `fn`
    |         help: `const` must come before `unsafe`: `const unsafe`
286 | }
286 | }
    | - the item list ends here
    |
    = note: keyword order for functions declaration is `default`, `pub`, `const`, `async`, `unsafe`, `extern`
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
error: could not compile `core` due to previous error
