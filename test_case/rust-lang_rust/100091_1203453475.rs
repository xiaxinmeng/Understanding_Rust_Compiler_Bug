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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_passes/src/errors.rs at line 555:
 #[error(passes::link_ordinal)]
 pub struct LinkOrdinal {
     #[primary_span]
-    pub attr_span: Span
+    pub attr_span: Span,
 
 #[derive(SessionDiagnostic)]
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 146:
                 | sym::stable
                 | sym::stable
                 | sym::rustc_allowed_through_unstable_modules
                 | sym::rustc_promotable => self.check_stability_promotable(&attr, span, target),
-                sym::link_ordinal => {
-                    self.check_link_ordinal(&attr, span, target)
-                },
+                sym::link_ordinal => self.check_link_ordinal(&attr, span, target),
             };
             };
             is_valid &= attr_is_valid;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_monomorphize/src/partitioning/merging.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/default.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/mod.rs" "/checkout/compiler/rustc_monomorphize/src/lib.rs" "/checkout/compiler/rustc_passes/src/errors.rs" "/checkout/compiler/rustc_plugin_impl/src/lib.rs" "/checkout/compiler/rustc_ast/src/token.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'thread '<unnamed>' panicked at '<unnamed>tx.send(entry.into_path()) failed with sending on a closed channel', ' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channelformat.rs:', format.rs166::17166
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31

Build completed unsuccessfully in 0:00:12
