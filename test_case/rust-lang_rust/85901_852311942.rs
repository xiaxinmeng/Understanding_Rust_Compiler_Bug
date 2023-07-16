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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/io/mod.rs at line 262:
 
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use self::buffered::IntoInnerError;
-#[stable(feature = "rust1", since = "1.0.0")]
-pub use self::buffered::{BufReader, BufWriter, LineWriter};
 #[unstable(feature = "bufwriter_into_raw_parts", issue = "80690")]
 pub use self::buffered::WriterPanicked;
+#[stable(feature = "rust1", since = "1.0.0")]
+pub use self::buffered::{BufReader, BufWriter, LineWriter};
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use self::copy::copy;
 #[stable(feature = "rust1", since = "1.0.0")]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/collections/hash/mod.rs" "/checkout/library/std/src/collections/mod.rs" "/checkout/library/std/src/keyword_docs.rs" "/checkout/library/std/src/panic/tests.rs" "/checkout/library/std/src/io/prelude.rs" "/checkout/library/std/src/io/impls/tests.rs" "/checkout/library/std/src/io/mod.rs" "/checkout/library/std/src/time/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
