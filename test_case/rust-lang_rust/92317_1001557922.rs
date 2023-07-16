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
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc/src/main.rs at line 46:
         // linking, so we need to explicitly depend on the function.
         #[cfg(target_os = "macos")]
         #[used]
-        static _F7: unsafe extern "C" fn() =
-            mimallocate_sys::_mi_macos_override_malloc;
+        static _F7: unsafe extern "C" fn() = mimallocate_sys::_mi_macos_override_malloc;
 
     rustc_driver::set_sigpipe_handler();
     rustc_driver::set_sigpipe_handler();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/proc_macro/src/bridge/buffer.rs" "/checkout/library/proc_macro/src/bridge/closure.rs" "/checkout/library/proc_macro/src/bridge/handle.rs" "/checkout/library/proc_macro/src/bridge/server.rs" "/checkout/library/proc_macro/src/bridge/client.rs" "/checkout/library/proc_macro/src/bridge/mod.rs" "/checkout/compiler/rustc/src/main.rs" "/checkout/library/proc_macro/src/bridge/scoped_cell.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
