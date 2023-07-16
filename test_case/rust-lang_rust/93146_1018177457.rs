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
Diff in /checkout/library/std/src/lib.rs at line 512:
 #[unstable(feature = "once_cell", issue = "74465")]
 pub mod lazy;
-
-
 // Pull in `std_float` crate  into libstd. The contents of
 // `std_float` are in a different repository: rust-lang/portable-simd.
 #[path = "../../portable-simd/crates/std_float/src/lib.rs"]
Diff in /checkout/library/std/src/lib.rs at line 527:
 #[unstable(feature = "portable_simd", issue = "86656")]
 pub mod simd {
     #[doc(inline)]
-    pub use core::simd::*;
-    #[doc(inline)]
     pub use crate::std_float::StdFloat;
+    #[doc(inline)]
+    pub use core::simd::*;
 
 
 #[stable(feature = "futures_api", since = "1.36.0")]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/lib.rs" "/checkout/library/std/src/lazy.rs" "/checkout/library/std/src/primitive_docs.rs" "/checkout/library/std/src/prelude/mod.rs" "/checkout/library/std/src/sys_common/memchr/tests.rs" "/checkout/library/std/src/sys_common/tests.rs" "/checkout/library/std/src/sys_common/io.rs" "/checkout/library/std/src/ffi/os_str/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
