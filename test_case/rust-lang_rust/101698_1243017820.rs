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
 use crate::fmt;
 use crate::intrinsics;
-use crate::cmp::Ordering;
 
 ///////////////////////////////////////////////////////////////////////////////
 // Any trait
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/num/dec2flt/float.rs" "/checkout/library/core/src/num/dec2flt/common.rs" "/checkout/library/core/src/num/bignum.rs" "/checkout/library/core/src/cell/once.rs" "/checkout/library/core/src/num/nonzero.rs" "/checkout/library/core/src/any.rs" "/checkout/library/core/src/num/flt2dec/mod.rs" "/checkout/library/core/src/num/dec2flt/fpu.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
