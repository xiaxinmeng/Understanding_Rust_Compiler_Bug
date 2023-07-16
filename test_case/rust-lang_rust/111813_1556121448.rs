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
  Stored in directory: /tmp/pip-ephem-wheel-cache-crotg9do/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:95c567d2b3a0a9798f2c6b2b40760075136d4d025da5b998a7ad9bf87adc6302
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7fae8e8dbe50>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 6:
 
 use crate::MirPass;
 use rustc_index::{bit_set::BitSet, IndexSlice, IndexVec};
+use rustc_middle::mir::visit::{MutVisitor, PlaceContext, Visitor};
 use rustc_middle::mir::*;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/dest_prop.rs" "/checkout/compiler/rustc_mir_transform/src/large_enums.rs" "/checkout/compiler/rustc_mir_transform/src/early_otherwise_branch.rs" "/checkout/compiler/rustc_mir_transform/src/elaborate_box_derefs.rs" "/checkout/compiler/rustc_mir_transform/src/remove_storage_markers.rs" "/checkout/compiler/rustc_mir_transform/src/prettify.rs" "/checkout/compiler/rustc_mir_transform/src/remove_noop_landing_pads.rs" "/checkout/compiler/rustc_mir_transform/src/errors.rs"` failed.
 use rustc_middle::ty::TyCtxt;
-use rustc_middle::mir::visit::{MutVisitor, PlaceContext, Visitor};
 
 pub struct ReorderBasicBlocks;
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 25:
             return;
             return;
         }
 
-        let mut updater = BasicBlockUpdater {
-            map: rpo.invert_bijective_mapping(),
-        };
-        };
+        let mut updater = BasicBlockUpdater { map: rpo.invert_bijective_mapping(), tcx };
         debug_assert_eq!(updater.map[START_BLOCK], START_BLOCK);
         updater.visit_body(body);
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 44:
     }
 
 
     fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
-        let mut finder = LocalFinder {
-            map: IndexVec::new(),
-            seen: BitSet::new_empty(body.local_decls.len()),
+        let mut finder =
+        let mut finder =
+            LocalFinder { map: IndexVec::new(), seen: BitSet::new_empty(body.local_decls.len()) };
         // We can't reorder the return place or the arguments
         // We can't reorder the return place or the arguments
         for i in 0..=body.arg_count {
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 68:
         }
 
 
-        let mut updater = LocalUpdater {
-            map: finder.map.invert_bijective_mapping(),
-        };
-        };
+        let mut updater = LocalUpdater { map: finder.map.invert_bijective_mapping(), tcx };
         debug_assert_eq!(updater.map[RETURN_PLACE], RETURN_PLACE);
         updater.visit_body_preserves_cfg(body);
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 98:
         self.tcx
     }
 
 
-    fn visit_terminator(
-        &mut self,
-        terminator: &mut Terminator<'tcx>,
-        _location: Location
-    ) {
+    fn visit_terminator(&mut self, terminator: &mut Terminator<'tcx>, _location: Location) {
         for succ in terminator.successors_mut() {
             *succ = self.map[*succ];
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 125:
Diff in /checkout/compiler/rustc_mir_transform/src/prettify.rs at line 125:
 impl<'tcx> Visitor<'tcx> for LocalFinder {
     fn visit_local(&mut self, l: Local, context: PlaceContext, _location: Location) {
         if context.is_use() {
-           self.track(l);
+            self.track(l);
     }
 }
