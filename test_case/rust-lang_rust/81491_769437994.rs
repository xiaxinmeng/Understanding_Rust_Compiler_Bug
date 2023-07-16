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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 3637:
             _ => None,
         })
         .expect("Expected associated type binding");
-    debug!("Render deref methods for {:#?}, target {:#?}",
-        impl_.inner_impl().for_,
-        target);
+    debug!("Render deref methods for {:#?}, target {:#?}", impl_.inner_impl().for_, target);
     let what =
         AssocItemRender::DerefFor { trait_: deref_type, type_: real_target, deref_mut_: deref_mut };
     if let Some(did) = target.def_id_full(cx.cache()) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:19
