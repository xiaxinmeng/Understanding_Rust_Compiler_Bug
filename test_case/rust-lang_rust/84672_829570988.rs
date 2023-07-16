
error[E0559]: variant `rustc_target::spec::abi::Abi::C` has no field named `unwind`
  --> src/tools/miri/src/shims/posix/foreign_items.rs:47:41
   |
47 |                 check_abi(abi, Abi::C { unwind: false })?;
   |                                         ^^^^^^ `rustc_target::spec::abi::Abi::C` does not have this field
