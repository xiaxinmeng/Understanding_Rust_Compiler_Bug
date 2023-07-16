plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Diff in /checkout/library/alloc/src/slice.rs at line 92:
 use crate::boxed::Box;
 use crate::vec::Vec;
 
+#[unstable(feature = "slice_range", issue = "76393")]
+pub use core::slice::range;
 #[unstable(feature = "array_chunks", issue = "74985")]
 pub use core::slice::ArrayChunks;
 #[unstable(feature = "array_chunks", issue = "74985")]
Diff in /checkout/library/alloc/src/slice.rs at line 120:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/slice.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 pub use core::slice::{RSplit, RSplitMut};
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use core::slice::{RSplitN, RSplitNMut, SplitN, SplitNMut};
-#[unstable(feature = "slice_range", issue = "76393")]
-pub use core::slice::range;
 
 ////////////////////////////////////////////////////////////////////////////////
 // Basic slice extension methods
Build completed unsuccessfully in 0:00:20
