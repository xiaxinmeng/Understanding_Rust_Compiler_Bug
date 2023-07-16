plain
Successfully built a8dc212d7aa8
Successfully tagged rust-ci:latest
Built container sha256:a8dc212d7aa8bc22afb57b1157091ad2a8b4a849285aec06c469b747efea7c73
Uploading finished image to https://ci-caches.rust-lang.org/docker/aa85b52f727783ce661c5275c3edac7e8b4fbba025fc0a8e6392d6566595f3d157bb7da4c70e2eda4e5e6e7455d8337e7dfb6de6b288fcd6e68cf37f838a32b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/aa85b52f727783ce661c5275c3edac7e8b4fbba025fc0a8e6392d6566595f3d157bb7da4c70e2eda4e5e6e7455d8337e7dfb6de6b288fcd6e68cf37f838a32b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1631:
                         let name = it.name.as_ref().unwrap();
                         let item_type = it.type_();
                         let file_name = &item_path(item_type, &name.as_str());
-                            .borrow_mut()
-                            .borrow_mut()
-                            .insert(format!("{}{}", item_p, file_name), path);
+                        redirections.borrow_mut().insert(format!("{}{}", item_p, file_name), path);
                     }
                     None => return layout::redirect(&format!("{}{}", self.root_path(), path)),
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
