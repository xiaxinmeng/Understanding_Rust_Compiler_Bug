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
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180117 sha256=a2b5d39c8ff2686626c851b00c3d3ae10157feb2cc6d0d07e414234b479dbb17
  Stored in directory: /tmp/pip-ephem-wheel-cache-8vdogy9k/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 829a00b40300
Successfully tagged rust-ci:latest
Built container sha256:829a00b40300cb77240fd44ec64cc7bb867957904112d7d94b9d6b540c2a1ef5
Uploading finished image to https://ci-caches.rust-lang.org/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d
upload failed: - to s3://rust-lang-ci-sccache2/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/core/src/iter/adapters/flatten.rs at line 1:
 use crate::fmt;
-use crate::iter::{DoubleEndedIterator, Fuse, FusedIterator, Iterator, Map, TrustedLen, Once};
+use crate::iter::{DoubleEndedIterator, Fuse, FusedIterator, Iterator, Map, Once, TrustedLen};
 use crate::ops::{ControlFlow, Try};
 use crate::result;
Diff in /checkout/library/core/src/iter/adapters/flatten.rs at line 671:
     fn spec_size(&self) -> (usize, Option<usize>);
 }
 }
 
-impl<I, U> SpecOnceIter for FlattenCompat<I, U> where
+impl<I, U> SpecOnceIter for FlattenCompat<I, U>
+where
     I: Iterator<Item: IntoIterator<IntoIter = U, Item = U::Item>>,
     U: Iterator,
Diff in /checkout/library/core/src/iter/adapters/flatten.rs at line 711:
     }
 }
 
 
-impl<I, U> SpecOnceIter for FlattenCompat<I, U> where
+impl<I, U> SpecOnceIter for FlattenCompat<I, U>
+where
     I: Iterator<Item: IntoIterator<IntoIter = U, Item = U::Item>>,
     U: Iterator + AtMostOnceIter,
Diff in /checkout/library/core/src/iter/adapters/flatten.rs at line 732:
         (0, upper)
     }
 }
 }
-
-
 
 #[doc(hidden)]
 #[unstable(feature = "std_internals", issue = "none")]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/cloned.rs" "/checkout/library/core/src/iter/adapters/array_chunks.rs" "/checkout/library/core/src/iter/adapters/skip.rs" "/checkout/library/core/src/iter/adapters/filter_map.rs" "/checkout/library/core/src/iter/adapters/flatten.rs" "/checkout/library/core/src/iter/adapters/enumerate.rs" "/checkout/library/core/src/iter/adapters/step_by.rs" "/checkout/library/core/src/iter/adapters/map_while.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
