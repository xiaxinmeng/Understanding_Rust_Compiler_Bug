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
Diff in /checkout/library/core/src/str/mod.rs at line 70:
 pub use iter::SplitInclusive;
 
 #[unstable(feature = "unicode_converter", issue = "none")]
-pub use iter::{CharsUppercase, CharsLowercase};
+pub use iter::{CharsLowercase, CharsUppercase};
 
 #[unstable(feature = "str_internals", issue = "none")]
 pub use validations::{next_code_point, utf8_char_width};
Diff in /checkout/library/core/src/str/mod.rs at line 842:
     pub fn char_indices(&self) -> CharIndices<'_> {
         CharIndices { front_offset: 0, iter: self.chars() }
-
 
 
     /// Returns an iterator over the [`char`]s of a string slice,
     /// converted to uppercase.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/str/pattern.rs" "/checkout/library/core/src/str/validations.rs" "/checkout/library/core/src/str/error.rs" "/checkout/library/core/src/str/count.rs" "/checkout/library/core/src/str/lossy.rs" "/checkout/library/core/src/str/iter.rs" "/checkout/library/core/src/str/mod.rs" "/checkout/library/core/src/str/converts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
