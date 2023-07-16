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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/boxed/thin.rs at line 2:
 // https://github.com/matthieu-m/rfc2580/blob/b58d1d3cba0d4b5e859d3617ea2d0943aaa31329/examples/thin.rs
 // by matthieu-m
 use crate::alloc::{self, Layout, LayoutError};
-use core::hint::unreachable_unchecked;
 use core::fmt::{self, Debug, Display, Formatter};
+use core::hint::unreachable_unchecked;
 use core::marker::{PhantomData, Unsize};
 use core::mem;
 use core::ops::{Deref, DerefMut};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/benches/btree/set.rs" "/checkout/library/alloc/src/boxed/thin.rs" "/checkout/library/alloc/src/rc/tests.rs" "/checkout/library/alloc/src/task.rs" "/checkout/library/alloc/src/boxed.rs" "/checkout/library/alloc/benches/vec.rs" "/checkout/library/alloc/benches/string.rs" "/checkout/library/alloc/src/string.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
