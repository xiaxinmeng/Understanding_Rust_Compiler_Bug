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
  Stored in directory: /tmp/pip-ephem-wheel-cache-fyjzey1b/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:e862091a8911db0196db3acbe3cb067f59527c206b3aaf54c5d8e6e272393fbe
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7efdd57dd550>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 15.34s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_monomorphize/src/partitioning/merging.rs at line 49:
         let mut consumed_cgu_names = cgu_contents.remove(&cgu_n.name()).unwrap();
         cgu_contents.get_mut(&cgu_n_minus_1.name()).unwrap().append(&mut consumed_cgu_names);
-        debug!(
-        debug!(
-            "CodegenUnit {} merged into CodegenUnit {}",
-            cgu_n.name(),
-            cgu_n_minus_1.name()
-        );
+        debug!("CodegenUnit {} merged into CodegenUnit {}", cgu_n.name(), cgu_n_minus_1.name());
 
 
     let cgu_name_builder = &mut CodegenUnitNameBuilder::new(cx.tcx);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_monomorphize/src/partitioning/merging.rs" "/checkout/compiler/rustc_monomorphize/src/collector.rs" "/checkout/compiler/rustc_monomorphize/src/lib.rs" "/checkout/compiler/rustc_monomorphize/src/util.rs" "/checkout/compiler/rustc_monomorphize/src/errors.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/default.rs" "/checkout/library/sysroot/src/lib.rs" "/checkout/library/test/src/helpers/shuffle.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
