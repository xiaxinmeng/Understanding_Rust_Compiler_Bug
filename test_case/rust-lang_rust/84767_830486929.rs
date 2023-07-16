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
Diff in /checkout/library/core/src/ops/try_trait.rs at line 121:
 #[unstable(feature = "try_trait_v2", issue = "84277")]
 #[rustc_on_unimplemented(
     on(
-        all(
-            from_method = "from_output",
-            from_desugaring = "TryBlock"
-        ),
+        all(from_method = "from_output", from_desugaring = "TryBlock"),
         message = "a `try` block must return `Result` or `Option` \
                     (or another type that implements `{Try}`)",
         label = "could not wrap the final value of the block as `{Self}` doesn't implement `Try`",
Diff in /checkout/library/core/src/ops/try_trait.rs at line 131:
     on(
-        all(
-        all(
-            from_method = "branch",
-            from_desugaring = "QuestionMark"
-        ),
+        all(from_method = "branch", from_desugaring = "QuestionMark"),
         message = "the `?` operator can only be applied to values \
                     that implement `{Try}`",
         label = "the `?` operator cannot be applied to type `{Self}`"
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/ops/try.rs" "/checkout/library/core/src/ops/index.rs" "/checkout/library/core/src/ops/try_trait.rs" "/checkout/library/core/src/ops/unsize.rs" "/checkout/library/core/src/ops/range.rs" "/checkout/library/core/src/ops/drop.rs" "/checkout/library/core/src/ops/generator.rs" "/checkout/library/core/src/ops/deref.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
