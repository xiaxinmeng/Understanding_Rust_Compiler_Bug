plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180118 sha256=2f37b3a1d0677cd3228b6c9d3baf1ad4a6f9d44f04404765e8544fc47767cbcc
  Stored in directory: /tmp/pip-ephem-wheel-cache-eqnotk9t/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 1ed93344f481
Successfully tagged rust-ci:latest
Built container sha256:1ed93344f48148f8a0eb6b66d758bf4872dad0de233887d2cb7cc54a001edfb0
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
-                                    Rvalue::Cast(
-                                        CastKind::Transmute,
-                                        args.next().unwrap(),
-                                        dst_ty,
-                                )))),
+                                    Rvalue::Cast(CastKind::Transmute, args.next().unwrap(), dst_ty),
+                                ))),
-                            assert_eq!(
-                                args.next(),
-                                None,
-                                "Extra argument for transmute intrinsic"
-                                "Extra argument for transmute intrinsic"
-                            );
+                            assert_eq!(args.next(), None, "Extra argument for transmute intrinsic");
                             drop(args);
                             terminator.kind = TerminatorKind::Goto { target };
Diff in /checkout/compiler/rustc_mir_transform/src/lower_intrinsics.rs at line 188:
Diff in /checkout/compiler/rustc_mir_transform/src/lower_intrinsics.rs at line 188:
-                            debug_assert!(!matches!(dst_ty.inhabited_predicate(tcx), InhabitedPredicate::True));
+                            debug_assert!(!matches!(
+                                dst_ty.inhabited_predicate(tcx),
+                                InhabitedPredicate::True
+                            ));
                             // `transmute::<_, !>(x)` is UB for anything inhabited,
                             // and must be unreachable if `x` is uninhabited.
                             terminator.kind = TerminatorKind::Unreachable;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_typeck/src/coercion.rs" "/checkout/compiler/rustc_ast_passes/src/show_span.rs" "/checkout/compiler/rustc_ast_passes/src/feature_gate.rs" "/checkout/compiler/rustc_mir_transform/src/remove_storage_markers.rs" "/checkout/compiler/rustc_mir_transform/src/lower_intrinsics.rs" "/checkout/compiler/rustc_index/src/lib.rs" "/checkout/compiler/rustc_hir/src/def_path_hash_map.rs" "/checkout/compiler/rustc_hir_typeck/src/check.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
