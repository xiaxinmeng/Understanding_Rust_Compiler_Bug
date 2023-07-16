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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_log/src/lib.rs at line 98:
 
     let error_layer = tracing_error::ErrorLayer::default();
 
-    let subscriber = tracing_subscriber::Registry::default().with(error_layer.with_filter(error_filter)).with(log_layer.with_filter(log_filter));
+    let subscriber = tracing_subscriber::Registry::default()
+        .with(error_layer.with_filter(error_filter))
+        .with(log_layer.with_filter(log_filter));
     tracing::subscriber::set_global_default(subscriber).unwrap();
     Ok(())
     Ok(())
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/abs_domain.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/builder.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/mod.rs" "/checkout/compiler/rustc_llvm/src/lib.rs" "/checkout/compiler/rustc_llvm/build.rs" "/checkout/compiler/rustc_log/src/lib.rs" "/checkout/compiler/rustc_monomorphize/src/polymorphize.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/fmt.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
