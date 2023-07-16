plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:90:33
    |
86  |   macro_rules! assert_ne {
    |   ---------------------- in this expansion of `assert_ne!` (#2)
...
90  |                   if *left_val == *right_val {
    |                                   ^^^^^^^^^^ expected enum `LinkerFlavorCli`, found enum `LinkerFlavor`
   ::: compiler/rustc_interface/src/tests.rs:536:5
    |
536 | /     macro_rules! untracked {
536 | /     macro_rules! untracked {
537 | |         ($name: ident, $non_default_value: expr) => {
538 | |             assert_ne!(opts.cg.$name, $non_default_value);
    | |             --------------------------------------------- in this macro invocation (#2)
539 | |             opts.cg.$name = $non_default_value;
540 | |             assert_same_hash(&reference, &opts);
542 | |     }
542 | |     }
    | |_____- in this expansion of `untracked!` (#1)
...
555 |       untracked!(linker_flavor, Some(LinkerFlavor::Gcc));
    |
    |
    = note: expected enum `Option<LinkerFlavorCli>`
               found enum `Option<LinkerFlavor>`
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:555:36
    |
    |
555 |     untracked!(linker_flavor, Some(LinkerFlavor::Gcc));
    |                               ---- ^^^^^^^^^^^^^^^^^ expected enum `LinkerFlavorCli`, found enum `LinkerFlavor`
    |                               arguments to this enum variant are incorrect
    |
note: tuple variant defined here
   --> /checkout/library/core/src/option.rs:526:5
