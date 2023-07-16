
error: unexpected expression: `{
           let res =
               ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[":dep nalgebra_latex = { version = \"",
                                   "\", features = [\"lin_sys\", \"evcxr\"] }"],
                       &[::core::fmt::ArgumentV1::new_display(&"1")]));
           res
       }`
 --> foo.rs:1:9
  |
1 | ... = format!(":dep nalgebra_latex = {{ version = \"{}\", features = [\"lin_sys\", \"evcxr\"] }}", env!("LOL"...
  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
