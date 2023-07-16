plain
    Checking gsgdt v0.1.2
    Checking tracing-serde v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
error[E0596]: cannot borrow `has_feature` as mutable, as it is not declared as mutable
   |
   |
92 |     has_feature: impl FnMut(&str) -> bool,
   |     ----------- help: consider changing this to be mutable: `mut has_feature`
...
95 |     if has_feature("thumb-mode") && !has_feature("thumb2") {
   |        ^^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `has_feature` as mutable, as it is not declared as mutable
   |
   |
92 |     has_feature: impl FnMut(&str) -> bool,
   |     ----------- help: consider changing this to be mutable: `mut has_feature`
...
95 |     if has_feature("thumb-mode") && !has_feature("thumb2") {
   |                                      ^^^^^^^^^^^ cannot borrow as mutable
    Checking tracing-subscriber v0.2.16
For more information about this error, try `rustc --explain E0596`.
error: could not compile `rustc_target` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
