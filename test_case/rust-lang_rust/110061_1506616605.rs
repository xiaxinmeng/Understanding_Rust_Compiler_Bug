plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:83b7061638ee4956cf7545a6f7efe594e5ad0247)
Download action repository 'rust-lang/simpleinfra@master' (SHA:f913b2b4d76b327031c3be225d54dc7e8a940241)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-pkdvhu45/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 9bec5b5bc00e
Successfully tagged rust-ci:latest
Built container sha256:9bec5b5bc00eb6bcd62272e870a548388e4340b6c185e5ba92bf84159225e10a
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
    Finished release [optimized] target(s) in 13.69s
fmt check
Diff in /checkout/compiler/rustc_hir/src/hir.rs at line 3533:
         match self {
             OwnerNode::Item(Item {
                 kind:
-                    ItemKind::Static(_, _, body)
-                    | ItemKind::Const(_, body)
-                    | ItemKind::Fn(_, _, body),
+                    ItemKind::Static(_, _, body) | ItemKind::Const(_, body) | ItemKind::Fn(_, _, body),
             })
             })
             | OwnerNode::TraitItem(TraitItem {
Diff in /checkout/compiler/rustc_hir/src/hir.rs at line 3542:
                 kind:
-                    TraitItemKind::Fn(_, TraitFn::Provided(body))
-                    | TraitItemKind::Const(_, Some(body)),
+                    TraitItemKind::Fn(_, TraitFn::Provided(body)) | TraitItemKind::Const(_, Some(body)),
             })
             })
             | OwnerNode::ImplItem(ImplItem {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/lib.rs" "/checkout/compiler/rustc_hir/src/def_path_hash_map.rs" "/checkout/compiler/rustc_mir_dataflow/src/rustc_peek.rs" "/checkout/compiler/rustc_hir/src/hir_id.rs" "/checkout/compiler/rustc_hir/src/stable_hash_impls.rs" "/checkout/compiler/rustc_hir/src/hir.rs" "/checkout/compiler/rustc_mir_dataflow/src/un_derefer.rs" "/checkout/compiler/rustc_mir_dataflow/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
