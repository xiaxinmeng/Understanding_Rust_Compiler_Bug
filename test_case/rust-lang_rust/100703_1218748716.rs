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
-        self.entries.push((
-            name.as_bytes().to_vec(),
-            ArchiveEntry::File(file.to_owned()),
-        ));
+        self.entries.push((name.as_bytes().to_vec(), ArchiveEntry::File(file.to_owned())));
 
     fn add_archive(
     fn add_archive(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs" "/checkout/compiler/rustc_codegen_cranelift/src/archive.rs" "/checkout/compiler/rustc_codegen_cranelift/src/pretty_clif.rs" "/checkout/compiler/rustc_codegen_cranelift/src/base.rs" "/checkout/compiler/rustc_codegen_cranelift/src/inline_asm.rs" "/checkout/compiler/rustc_codegen_cranelift/src/constant.rs" "/checkout/compiler/rustc_codegen_cranelift/src/linkage.rs" "/checkout/compiler/rustc_codegen_cranelift/src/abi/returning.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
