plain
Successfully built fcdca6584b31
Successfully tagged rust-ci:latest
Built container sha256:fcdca6584b31b4a5744437aa839dabf7525e8da2d68da3148bef059521158fe1
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
         }
     }
 
-
     for m in &codegen_results.modules {
         if let Some(obj) = m.object.as_ref() {
             ab.add_file(obj);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/traits/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/command.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/statics.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/asm.rs" "/checkout/compiler/rustc_codegen_ssa/src/coverageinfo/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/debuginfo.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
