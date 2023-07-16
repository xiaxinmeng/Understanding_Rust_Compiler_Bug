plain
Successfully built 5ba6fff51151
Successfully tagged rust-ci:latest
Built container sha256:5ba6fff51151f3a2d73d2398e66d860ce8f7cd5de86bc426ac259df1d6b3d950
Uploading finished image to https://ci-caches.rust-lang.org/docker/dc5e36fabab9a418dcd1388689e70a41ae6bbea7195d4bd033228b0312de39f9e1d1fdc941775cd0a93a74a0110ae5c291a6c8ed0f0bf9425f4848a8035010bf
upload failed: - to s3://rust-lang-ci-sccache2/docker/dc5e36fabab9a418dcd1388689e70a41ae6bbea7195d4bd033228b0312de39f9e1d1fdc941775cd0a93a74a0110ae5c291a6c8ed0f0bf9425f4848a8035010bf Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
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
Diff in /checkout/compiler/rustc_mir_transform/src/scalar_deref_prop.rs at line 356:
                     place = target.project_deeper(&place.projection[1..], tcx);
                 }
                 assert_ne!(place.local, local);
-                if is_constant(place) {
-                    Value::Pointer(place)
-                    Value::Unknown
-                }
-                }
+                if is_constant(place) { Value::Pointer(place) } else { Value::Unknown }
             }
             // We do not know what to do, so mark as not-a-pointer.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/lib.rs" "/checkout/compiler/rustc_mir_transform/src/scalar_deref_prop.rs" "/checkout/compiler/rustc_middle/src/dep_graph/dep_node.rs" "/checkout/compiler/rustc_middle/src/thir/visit.rs" "/checkout/compiler/rustc_middle/src/query/mod.rs" "/checkout/compiler/rustc_mir_transform/src/dead_store_elimination.rs" "/checkout/compiler/rustc_middle/src/arena.rs" "/checkout/compiler/rustc_mir_transform/src/const_debuginfo.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
             _ => Value::Unknown,
