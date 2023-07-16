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
+    borrow::Borrow,
     cmp,
     fmt::Debug,
     iter,
Diff in /checkout/compiler/rustc_target/src/abi/layout.rs at line 6:
-    ops::{Bound, Deref}, borrow::Borrow,
+    ops::{Bound, Deref},
 
 
 use rand::{seq::SliceRandom, SeedableRng};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/sparc.rs" "/checkout/compiler/rustc_target/src/abi/call/msp430.rs" "/checkout/compiler/rustc_target/src/abi/call/mod.rs" "/checkout/compiler/rustc_target/src/abi/call/arm.rs" "/checkout/compiler/rustc_target/src/abi/call/x86_win64.rs" "/checkout/compiler/rustc_target/src/abi/layout.rs" "/checkout/compiler/rustc_target/src/abi/mod.rs" "/checkout/compiler/rustc_target/src/abi/call/mips.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
