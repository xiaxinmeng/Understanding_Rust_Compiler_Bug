plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0390]: only a single inherent implementation marked with `#[lang = "error_alloc_impl"]` is allowed for the `dyn error` primitive
    |
    |
255 | / impl dyn Error + 'static {
256 | |     /// Returns `true` if the boxed type is the same as `T`
257 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
...   |
292 | |     }
293 | | }
    | |_^
