plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-bde1q1h_/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 235f65817c92
Successfully tagged rust-ci:latest
Built container sha256:235f65817c92dbe3f0bc3a6239a8a98df2625ff26e526ed95fb41b74072cafd9
Uploading finished image to https://ci-caches.rust-lang.org/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d
upload failed: - to s3://rust-lang-ci-sccache2/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 14.82s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs at line 32:
 /// Functionality required for the bytes of an `Allocation`.
 pub trait AllocBytes:
-    Clone
-    + fmt::Debug
-    + fmt::Debug
-    + Eq
-    + PartialEq
-    + Hash
-    + Deref<Target = [u8]>
-    + DerefMut<Target = [u8]>
+    Clone + fmt::Debug + Eq + PartialEq + Hash + Deref<Target = [u8]> + DerefMut<Target = [u8]>
 {
     /// Adjust the bytes to the specified alignment -- by default, this is a no-op.
     fn adjust_to_align(self, _align: Align) -> Self;
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs at line 271:
 // The constructors are all without extra; the extra gets added by a machine hook later.
 impl<Prov: Provenance, Bytes: AllocBytes> Allocation<Prov, (), Bytes> {
     /// Creates an allocation from an existing `Bytes` value - this is needed for miri FFI support
-        bytes: Bytes,
-        align: Align,
-        mutability: Mutability,
-    ) -> Self {
-    ) -> Self {
+    pub fn from_raw_bytes(bytes: Bytes, align: Align, mutability: Mutability) -> Self {
         let size = Size::from_bytes(bytes.len());
             bytes,
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs at line 127:
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs at line 127:
 pub use self::value::{get_slice_bytes, ConstAlloc, ConstValue, Scalar};
 pub use self::allocation::{
 pub use self::allocation::{
-    alloc_range, AllocBytes, AllocError, AllocRange, AllocResult, Allocation, ConstAllocation, InitChunk,
-    InitChunkIter,
+    alloc_range, AllocBytes, AllocError, AllocRange, AllocResult, Allocation, ConstAllocation,
+    InitChunk, InitChunkIter,
 
 
 pub use self::pointer::{Pointer, PointerArithmetic, Provenance};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/middle/region.rs" "/checkout/compiler/rustc_middle/src/traits/select.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs" "/checkout/compiler/rustc_middle/src/mir/generic_graph.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/generic_graphviz.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/middle/codegen_fn_attrs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
