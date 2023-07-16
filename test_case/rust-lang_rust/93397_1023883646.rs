plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0522]: definition of an unknown language item: `slice_f32`
    --> library/core/src/slice/mod.rs:3986:1
     |
3986 | #[lang = "slice_f32"]
     | ^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `slice_f32`
error[E0522]: definition of an unknown language item: `slice_f64`
    --> library/core/src/slice/mod.rs:4015:1
     |
     |
4015 | #[lang = "slice_f64"]
     | ^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `slice_f64`

error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
     |
     |
3987 | / impl [f32] {
3988 | |     /// Sorts the slice of floats.
3989 | |     ///
3990 | |     /// This sort is in-place (i.e. does not allocate), *O*(*n* \* log(*n*)) worst-case, and uses
4012 | |     }
4013 | | }
     | |_^
     |
     |
     = help: consider using a trait to implement this method

error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
     |
     |
4016 | / impl [f64] {
4017 | |     /// Sorts the slice of floats.
4018 | |     ///
4019 | |     /// This sort is in-place (i.e. does not allocate), *O*(*n* \* log(*n*)) worst-case, and uses
4041 | |     }
4042 | | }
     | |_^
     |
