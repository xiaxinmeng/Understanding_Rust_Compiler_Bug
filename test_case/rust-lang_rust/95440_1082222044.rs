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
Diff in /checkout/src/bootstrap/tool.rs at line 386:
         let mut cmd = Command::new(builder.ensure(ErrorIndex { compiler }));
         let mut dylib_paths = builder.rustc_lib_paths(compiler);
         dylib_paths.push(PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host)));
-        add_dylib_path(dylib_paths, &mut cmd );
+        add_dylib_path(dylib_paths, &mut cmd);
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/install.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/bootstrap/metadata.rs" "/checkout/src/bootstrap/tool.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
