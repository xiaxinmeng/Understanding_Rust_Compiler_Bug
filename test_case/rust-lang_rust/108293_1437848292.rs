plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-vy09r4z8/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built d9071151170c
Successfully tagged rust-ci:latest
Built container sha256:d9071151170c1c3320b525d2c2380591d9895944b8aef0eab68384c4ad06b4d0
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 17.53s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_dataflow/src/framework/graphviz.rs at line 395:
         let mut afters = diffs.after.into_iter();
 
         let next_in_dataflow_order = |it: &mut std::vec::IntoIter<_>| {
-            if A::Direction::IS_FORWARD {
-                it.next().unwrap()
-            } else {
-                it.next_back().unwrap()
-            }
+            if A::Direction::IS_FORWARD { it.next().unwrap() } else { it.next_back().unwrap() }
 
 
         for (i, statement) in self.results.body()[block].statements.iter().enumerate() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/cleanup_post_borrowck.rs" "/checkout/compiler/rustc_mir_transform/src/elaborate_box_derefs.rs" "/checkout/compiler/rustc_mir_transform/src/lib.rs" "/checkout/compiler/rustc_mir_transform/src/remove_noop_landing_pads.rs" "/checkout/compiler/rustc_mir_transform/src/inline.rs" "/checkout/compiler/rustc_mir_transform/src/dead_store_elimination.rs" "/checkout/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/graphviz.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
