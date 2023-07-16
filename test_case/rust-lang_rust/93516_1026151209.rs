plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:90:33
    |
86  | / macro_rules! assert_ne {
87  | |     ($left:expr, $right:expr $(,)?) => ({
88  | |         match (&$left, &$right) {
89  | |             (left_val, right_val) => {
90  | |                 if *left_val == *right_val {
    | |                                 ^^^^^^^^^^ expected enum `Option`, found struct `BranchProtection`
112 | |     });
113 | | }
113 | | }
    | |_- in this expansion of `assert_ne!` (#2)
   ::: compiler/rustc_interface/src/tests.rs:706:5
    |
706 | /     macro_rules! tracked {
706 | /     macro_rules! tracked {
707 | |         ($name: ident, $non_default_value: expr) => {
708 | |             opts = reference.clone();
709 | |             assert_ne!(opts.debugging_opts.$name, $non_default_value);
...   |
712 | |         };
713 | |     }
    | |_____- in this expansion of `tracked!` (#1)
    | |_____- in this expansion of `tracked!` (#1)
...
722 | /     tracked!(
723 | |         branch_protection,
724 | |         BranchProtection { bti: true, pac_ret: Some(PacRet { leaf: true, key: PAuthKey::B }) }
    | |_____- in this macro invocation (#1)
    |
    = note: expected enum `Option<BranchProtection>`
             found struct `BranchProtection`
             found struct `BranchProtection`
help: try wrapping the expression in `Some`
    |
90  |                 if *left_val == Some(*right_val) {
    |                                 +++++          +
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:724:9
    |
    |
710 |             opts.debugging_opts.$name = $non_default_value;
...
...
724 |         BranchProtection { bti: true, pac_ret: Some(PacRet { leaf: true, key: PAuthKey::B }) }
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Option`, found struct `BranchProtection`
    = note: expected enum `Option<BranchProtection>`
             found struct `BranchProtection`
             found struct `BranchProtection`
help: try wrapping the expression in `Some`
    |
724 |         Some(BranchProtection { bti: true, pac_ret: Some(PacRet { leaf: true, key: PAuthKey::B }) })
    |         +++++                                                                                      +
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_interface` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
