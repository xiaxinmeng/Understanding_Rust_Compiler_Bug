plain
   Compiling object v0.22.0
   Compiling hashbrown v0.9.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error[E0432]: unresolved import `crate::detect::arch::is_x86_feature_detected`
    |
    |
304 |             use crate::detect::arch::is_x86_feature_detected;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `is_x86_feature_detected` in `std_detect::detect::arch`
    |
    = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
    |
304 |             use ::is_x86_feature_detected;


error[E0603]: module `arch` is private
    |
    |
304 |             use crate::detect::arch::is_x86_feature_detected;
    |                                ^^^^ private module
    |
note: the module `arch` is defined here
   --> library/std/src/../../stdarch/crates/std_detect/src/detect/mod.rs:32:9
32  |         mod arch;
    |         ^^^^^^^^^

error: aborting due to 2 previous errors
