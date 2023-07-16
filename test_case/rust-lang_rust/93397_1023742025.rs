plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.66
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
     |
     |
3986 | / impl [f32] {
3987 | |     /// Sorts the slice of floats.
3988 | |     ///
3989 | |     /// This sort is in-place (i.e. does not allocate), *O*(*n* \* log(*n*)) worst-case, and uses
4010 | |     }
4011 | | }
     | |_^
     |
     |
     = help: consider using a trait to implement this method

error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
     |
     |
4013 | / impl [f64] {
4014 | |     /// Sorts the slice of floats.
4015 | |     ///
4016 | |     /// This sort is in-place (i.e. does not allocate), *O*(*n* \* log(*n*)) worst-case, and uses
4037 | |     }
4038 | | }
     | |_^
     |
