plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    |
86  |   macro_rules! assert_ne {
    |   ---------------------- in this expansion of `assert_ne!` (#2)
...
90  |                   if *left_val == *right_val {
    |                                ^^ no implementation for `bool == Option<bool>`
   ::: compiler/rustc_interface/src/tests.rs:699:5
    |
699 | /     macro_rules! tracked {
699 | /     macro_rules! tracked {
700 | |         ($name: ident, $non_default_value: expr) => {
701 | |             opts = reference.clone();
702 | |             assert_ne!(opts.unstable_opts.$name, $non_default_value);
...   |
705 | |         };
706 | |     }
    | |_____- in this expansion of `tracked!` (#1)
    | |_____- in this expansion of `tracked!` (#1)
...
767 |       tracked!(plt, Some(true));
    |
    |
    = help: the trait `std::cmp::PartialEq<Option<bool>>` is not implemented for `bool`
    = help: the following other types implement trait `std::cmp::PartialEq<Rhs>`:
              <bool as std::cmp::PartialEq<rustc_target::json::Json>>
              <bool as std::cmp::PartialEq>
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:767:19
    |
    |
703 |             opts.unstable_opts.$name = $non_default_value;
...
...
767 |     tracked!(plt, Some(true));
    |                   ^^^^^^^^^^ expected `bool`, found enum `Option`
    = note: expected type `bool`
               found enum `Option<bool>`
help: use `Option::is_some` to test if the `Option` has a value
    |
    |
767 |     tracked!(plt, Some(true).is_some());

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_interface` due to 2 previous errors
