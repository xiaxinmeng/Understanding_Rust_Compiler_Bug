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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/ffi/mod.rs"` failed.
Diff in /checkout/library/std/src/ffi/mod.rs at line 150:
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use self::c_str::{CStr, CString, IntoStringError, NulError};
 
-#[stable(feature = "rust1", since = "1.0.0")]
-pub use self::os_str::{OsStr, OsString};
 #[stable(feature = "os_str_display", since = "1.51.0")]
 pub use self::os_str::Display;
+#[stable(feature = "rust1", since = "1.0.0")]
+pub use self::os_str::{OsStr, OsString};
 
 #[stable(feature = "core_c_void", since = "1.30.0")]
 pub use core::ffi::c_void;
Build completed unsuccessfully in 0:00:18
