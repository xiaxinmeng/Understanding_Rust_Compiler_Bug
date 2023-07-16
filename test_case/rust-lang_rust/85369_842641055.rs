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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 689:
         // Blacklist traits for which it would be nonsensical to suggest borrowing.
         // For instance, immutable references are always Copy, so suggesting to
         // borrow would always succeed, but it's probably not what the user wanted.
-        let blacklist: Vec<_> = [
-            LangItem::Copy,
-            LangItem::Clone,
-            LangItem::Unpin,
-            LangItem::Sized,
-            LangItem::Send,
-        .iter()
-        .iter()
-        .filter_map(|lang_item| self.tcx.lang_items().require(*lang_item).ok())
-        .collect();
+        let blacklist: Vec<_> =
+            [LangItem::Copy, LangItem::Clone, LangItem::Unpin, LangItem::Sized, LangItem::Send]
+                .iter()
+                .filter_map(|lang_item| self.tcx.lang_items().require(*lang_item).ok())
+                .collect();
 
         let span = obligation.cause.span;
         let param_env = obligation.param_env;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/structural_match.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/misc.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
