plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.16s
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
error: failed to parse the cfg from `rustc --print=cfg`, got:
___
lib___.rlib
lib___.so
lib___.so
lib___.a
lib___.so

bootstrap
debug_assertions
debug_assertions
panic="unwind"
target_abi=""
target_arch="x86_64"
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="fxsr"
target_feature="llvm14-builtins-abi"
target_feature="sse"
target_feature="sse2"
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
target_has_atomic_load_store="16"
target_has_atomic_load_store="32"
target_has_atomic_load_store="64"
target_has_atomic_load_store="8"
target_has_atomic_load_store="ptr"
target_os="linux"
target_pointer_width="64"
target_thread_local
target_vendor="unknown"


Caused by:
Caused by:
  failed to parse `` as a cfg expression: expected identifier, but cfg expression ended
