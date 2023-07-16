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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/dist.rs at line 1894:
 
     let copy = |src: &Path, dst, perms| {
         // Try to use a hard link for this file, but only if it's not already a symbolic link
-        if src.is_symlink() {
-            builder.install(src, dst, perms)
-            builder.copy(src, dst)
-        }
-        }
+        if src.is_symlink() { builder.install(src, dst, perms) } else { builder.copy(src, dst) }
 
 
     // On macOS, rustc (and LLVM tools) link to an unversioned libLLVM.dylib
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/lint.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/librustdoc/error.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs" "/checkout/src/bootstrap/run.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
