plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-bj9in9er/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 106df02f4a7e
Successfully tagged rust-ci:latest
Built container sha256:106df02f4a7e007d62ff3683c378f8e4b70843d1e588e97758b9118d7ad436ce
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.11s
fmt check
Diff in /checkout/src/librustdoc/html/format.rs at line 1229:
                     };
                     write!(self.inner, "{emit}")?;
                     self.state = Text;
+                }
                 // Read one character of an entity name
                 // Read one character of an entity name
                 (Entity(ref mut len, ref mut arr), c) => {
                     if *len as usize > arr.len() - 1 {
Diff in /checkout/src/librustdoc/html/format.rs at line 1237:
                     }
                     arr[*len as usize] = c as u8;
                     *len += 1;
+                }
+                }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/passes/stripper.rs" "/checkout/src/librustdoc/html/length_limit.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/visit_lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         }
         Ok(())
Build completed unsuccessfully in 0:00:14
