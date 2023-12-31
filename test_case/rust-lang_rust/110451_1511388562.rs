plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:4d706a8d448f000965ed9e883febfb36661202fe)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-2del39q4/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 79bf7f8939d7
Successfully tagged rust-ci:latest
Built container sha256:79bf7f8939d760adfe5eefb9db688e3a2100d1a19959c238323524ce06a48662
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.03s
Building tool tidy (stage0)
fmt check
Diff in /checkout/compiler/rustc_ast_lowering/src/lib.rs at line 368:
     krate: &'a Crate,
 ) -> IndexVec<LocalDefId, AstOwner<'a>> {
     let mut indexer = Indexer { node_id_to_def_id, index: IndexVec::new() };
-    *indexer.index.ensure_contains_elem(CRATE_DEF_ID, || AstOwner::NonOwner) = AstOwner::Crate(krate);
+    *indexer.index.ensure_contains_elem(CRATE_DEF_ID, || AstOwner::NonOwner) =
+        AstOwner::Crate(krate);
     visit::walk_crate(&mut indexer, krate);
     return indexer.index;
Diff in /checkout/compiler/rustc_ast_lowering/src/lib.rs at line 391:
 
 
         fn visit_assoc_item(&mut self, item: &'a ast::AssocItem, ctxt: visit::AssocCtxt) {
             let def_id = self.node_id_to_def_id[&item.id];
-            *self.index.ensure_contains_elem(def_id, || AstOwner::NonOwner) = AstOwner::AssocItem(item, ctxt);
+            *self.index.ensure_contains_elem(def_id, || AstOwner::NonOwner) =
+                AstOwner::AssocItem(item, ctxt);
             visit::walk_assoc_item(self, item, ctxt);
 
Diff in /checkout/compiler/rustc_ast_lowering/src/lib.rs at line 398:
Diff in /checkout/compiler/rustc_ast_lowering/src/lib.rs at line 398:
         fn visit_foreign_item(&mut self, item: &'a ast::ForeignItem) {
             let def_id = self.node_id_to_def_id[&item.id];
-            *self.index.ensure_contains_elem(def_id, || AstOwner::NonOwner) = AstOwner::ForeignItem(item);
+            *self.index.ensure_contains_elem(def_id, || AstOwner::NonOwner) =
+                AstOwner::ForeignItem(item);
             visit::walk_foreign_item(self, item);
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_type_ir/src/structural_impls.rs" "/checkout/compiler/rustc_query_impl/src/plumbing.rs" "/checkout/compiler/rustc_ast_lowering/src/errors.rs" "/checkout/compiler/rustc_query_impl/src/lib.rs" "/checkout/compiler/rustc_ast_lowering/src/lib.rs" "/checkout/compiler/rustc_query_impl/src/on_disk_cache.rs" "/checkout/compiler/rustc_ast_lowering/src/asm.rs" "/checkout/compiler/rustc_error_codes/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
