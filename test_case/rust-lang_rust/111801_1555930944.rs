plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:af9a3f1b7e12a54c737d8aa371acc8d05cb83a8f)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180115 sha256=c3e1cf75e5aea7348ad201d16633573bc9f3aa58c8e29a6ca459f306300d6c9d
  Stored in directory: /tmp/pip-ephem-wheel-cache-osy5r2jx/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:0f18cc57cee20413532a9dbe32720d2786b3f67492f4ff6a42d9f2fa0087d0b0
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7fbbbf24fcd0>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
-                    PrivateInterfacesLint {
-                        span,
-                        vis_descr,
-                        kind,
-                        descr: descr.into(),
-                        vis_span,
-                    },
+                    PrivateInterfacesLint { span, vis_descr, kind, descr: descr.into(), vis_span },
             } else {
             } else {
                 self.tcx.emit_spanned_lint(
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 2008:
                     lint::builtin::PRIVATE_BOUNDS,
                     hir_id,
-                    PrivateInterfacesLint {
-                        span,
-                        vis_descr,
-                        kind,
-                        kind,
-                        descr: descr.into(),
-                        vis_span,
-                    },
+                    PrivateInterfacesLint { span, vis_descr, kind, descr: descr.into(), vis_span },
             }
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_query_system/src/ich/impls_hir.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_privacy/src/errors.rs" "/checkout/compiler/rustc_session/src/options.rs" "/checkout/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_mir_transform/src/elaborate_drops.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/graph.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
