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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/dist.rs at line 1214:
 
         builder.ensure(compile::CodegenBackend { compiler, target, backend });
 
-        let mut tarball = Tarball::new(builder, &format!("rustc-codegen-{}", backend), &target.triple);
+        let mut tarball =
+            Tarball::new(builder, &format!("rustc-codegen-{}", backend), &target.triple);
         if backend == "cranelift" {
             tarball.set_overlay(OverlayKind::RustcCodegenCranelift);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/dist.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/src/bootstrap/dist.rs at line 1222:
         }
         }
         tarball.is_preview(true);
         tarball.add_legal_and_readme_to(format!("share/doc/rustc_codegen_{}", backend));
 
 
         let src = builder.sysroot(compiler);
         let backends_src = builder.sysroot_codegen_backends(compiler);
Build completed unsuccessfully in 0:00:20
