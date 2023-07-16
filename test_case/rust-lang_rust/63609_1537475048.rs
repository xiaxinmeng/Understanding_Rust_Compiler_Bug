
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -Cprofile-use=/tmp/pgo-data/merged.profdata --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 1)
  --- stdout
  ___
  lib___.rlib
  lib___.so
  lib___.so
  lib___.a
  lib___.so
  /home/nazar-pc/.rustup/toolchains/nightly-2023-04-02-x86_64-unknown-linux-gnu
  off
  packed
  unpacked
  ___
  debug_assertions
  panic="unwind"
  proc_macro
  target_abi=""
  target_arch="x86_64"
  target_endian="little"
  target_env="gnu"
  target_family="unix"
  target_feature="fxsr"
  target_feature="sse"
  target_feature="sse2"
  target_has_atomic
  target_has_atomic="16"
  target_has_atomic="32"
  target_has_atomic="64"
  target_has_atomic="8"
  target_has_atomic="ptr"
  target_has_atomic_equal_alignment="16"
  target_has_atomic_equal_alignment="32"
  target_has_atomic_equal_alignment="64"
  target_has_atomic_equal_alignment="8"
  target_has_atomic_equal_alignment="ptr"
  target_has_atomic_load_store
  target_has_atomic_load_store="16"
  target_has_atomic_load_store="32"
  target_has_atomic_load_store="64"
  target_has_atomic_load_store="8"
  target_has_atomic_load_store="ptr"
  target_os="linux"
  target_pointer_width="64"
  target_thread_local
  target_vendor="unknown"
  unix

  --- stderr
  error: file `/tmp/pgo-data/merged.profdata` passed to `-C profile-use` does not exist.

  error: aborting due to previous error
