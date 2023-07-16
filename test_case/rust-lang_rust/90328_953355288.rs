plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0390]: only a single inherent implementation marked with `#[lang = "error_alloc_impl"]` is allowed for the `dyn Error` primitive
    |
    |
254 | / impl dyn Error + 'static {
255 | |     /// Returns `true` if the boxed type is the same as `T`
256 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
...   |
291 | |     }
292 | | }
    | |_^
    | |_^
    |
    = help: consider using a trait to implement these methods

error[E0390]: only a single inherent implementation marked with `#[lang = "error_alloc_impl"]` is allowed for the `dyn Error` primitive
    |
    |
294 | / impl dyn Error + 'static + Send {
295 | |     /// Forwards to the method defined on the type `dyn Error`.
296 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
...   |
314 | |     }
315 | | }
    | |_^
    | |_^
    |
    = help: consider using a trait to implement these methods

error[E0390]: only a single inherent implementation marked with `#[lang = "error_alloc_impl"]` is allowed for the `dyn Error` primitive
    |
    |
317 | / impl dyn Error + 'static + Send + Sync {
318 | |     /// Forwards to the method defined on the type `dyn Error`.
319 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
...   |
337 | |     }
338 | | }
    | |_^
    | |_^
    |
    = help: consider using a trait to implement these methods

error[E0390]: only a single inherent implementation marked with `#[lang = "error_alloc_impl"]` is allowed for the `dyn Error` primitive
    |
    |
340 | / impl dyn Error {
341 | |     /// Returns an iterator starting with the current error and continuing with
342 | |     /// recursively calling [`Error::source`].
...   |
396 | |     }
397 | | }
    | |_^
