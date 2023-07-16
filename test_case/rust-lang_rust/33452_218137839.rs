
error: internal compiler error: ../src/librustc_mir/transform/promote_consts.rs:204: tmp4 not promotable: Unpromotable
   --> /Users/nox/src/servo/components/script/dom/bindings/interface.rs:122:23
122 |>                 name: b"Function\0" as *const _ as *const libc::c_char,
    |>                       ^^^^^^^^^^^^^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
