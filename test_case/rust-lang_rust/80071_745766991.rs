plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +741b293cc27a3555f1a8928075436352167e40a7:refs/remotes/pull/80071/merge
---
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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_trait_impls.rs"` failed.
Diff in /checkout/src/librustdoc/passes/collect_trait_impls.rs at line 16:
 
 crate fn collect_trait_impls(krate: Crate, cx: &DocContext<'_>) -> Crate {
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     let mut synth = SyntheticImplCollector::new(cx);
-    let mut krate = cx.sess().time("collect_synthetic_impls", || {
-        synth.fold_crate(krate)
-    });
+    let mut krate = cx.sess().time("collect_synthetic_impls", || synth.fold_crate(krate));
 
     let prims: FxHashSet<PrimitiveType> = krate.primitives.iter().map(|p| p.1).collect();
 
Diff in /checkout/src/librustdoc/passes/collect_trait_impls.rs at line 25:
     let crate_items = {
         let mut coll = ItemCollector::new();
-        krate = cx.sess().time("collect_items_for_trait_impls", || {
-            coll.fold_crate(krate)
-        });
+        krate = cx.sess().time("collect_items_for_trait_impls", || coll.fold_crate(krate));
         coll.items
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:21
== clock drift check ==
