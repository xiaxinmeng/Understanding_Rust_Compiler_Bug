plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 1603a70f82240ba2d27f72f964e36614d7620ad3 and bdbcf8f05ba7e512dc83e5139d7d874507b88308
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> src/tools/rls/racer/src/racer/ast.rs:1013:31
     |
1013 |         if let ItemKind::Enum(ref enum_definition, _) = i.kind {
     |                               ^^^^^^^^^^^^^^^^^^^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2880:10
     |
     |
2880 |     Enum(Box<Enum>),

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 1 field
    --> src/tools/rls/racer/src/racer/ast.rs:1046:29
     |
     |
1046 |             ItemKind::Const(_, ref ty, ref _expr) => self.ty = Ty::from_ast(ty, &self.scope),
     |                             ^  ^^^^^^  ^^^^^^^^^ expected 1 field, found 3
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2856:11
     |
     |
2856 |     Const(Box<Const>),

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 1 field
    --> src/tools/rls/racer/src/racer/ast.rs:1047:30
     |
     |
1047 |             ItemKind::Static(ref ty, m, ref _expr) => {
     |                              ^^^^^^  ^  ^^^^^^^^^ expected 1 field, found 3
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2852:12
     |
     |
2852 |     Static(Box<Static>),

For more information about this error, try `rustc --explain E0023`.
warning: `racer` (lib) generated 2 warnings
error: could not compile `racer` due to 3 previous errors; 2 warnings emitted
---
tests/pass/backtrace/backtrace-std.rs  ... ok
tests/pass/0weak_memory_consistency.rs  ... ok

tests/pass/integer-ops.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-1fd7bc8ace8b19d6.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-28b21595824ceb29.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-939b1abf4012e1ce.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-5891685030f268f5.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-02d594a0068c74d5.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-c0be874f2cc201b5.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-9b15165aa8ec1acf.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-a0ff605b584f67a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-f60083e028003752" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-4cab3f2a29ec0f70" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-193d9ef40856b5f0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-399f873fd86b1ba6" "tests/pass/integer-ops.rs" "--error-format=json" "-Coverflow-checks=off"
Pass got exit status: 1

actual output differed from expected tests/pass/integer-ops.stderr
Diff < left / right > :
---
test unit_tests::test_format_snippet ... ok
test test::verify_check_works_with_stdin ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/items.rs:1803:
             ast::ItemKind::Static(ref static_) => {
                 (None, "static", &static_.ty, static_.mutbl, &static_.expr)
             }
-            ast::ItemKind::Const(ref const_) => {
-                (Some(const_.defaultness), "const", &const_.ty, ast::Mutability::Not, &const_.expr)
-            }
+            ast::ItemKind::Const(ref const_) => (
+                Some(const_.defaultness),
+                "const",
+                &const_.ty,
+                &const_.expr,
+            ),
             _ => unreachable!(),
         };
         };
         StaticParts {

Mismatch at src/items.rs:1822:
 
     pub(crate) fn from_trait_item(ti: &'a ast::AssocItem) -> Self {
         match &ti.kind {
-            ast::AssocItemKind::Const(const_) => {
-                StaticParts {
-                    prefix: "const",
-                    vis: &ti.vis,
-                    ident: ti.ident,
-                    ty: &const_.ty,
-                    mutability: ast::Mutability::Not,
-                    expr_opt: const_.expr.as_ref(),
-                    defaultness: Some(const_.defaultness),
-                    span: ti.span,
-            }
-            }
+            ast::AssocItemKind::Const(const_) => StaticParts {
+                prefix: "const",
+                vis: &ti.vis,
+                ident: ti.ident,
+                ty: &const_.ty,
+                mutability: ast::Mutability::Not,
+                expr_opt: const_.expr.as_ref(),
+                defaultness: Some(const_.defaultness),
+                span: ti.span,
             _ => unreachable!(),
         }
     }


Mismatch at src/items.rs:1840:
 
     pub(crate) fn from_impl_item(ii: &'a ast::AssocItem) -> Self {
         match &ii.kind {
-            ast::AssocItemKind::Const(const_) => {
-                StaticParts {
-                    prefix: "const",
-                    vis: &ii.vis,
-                    ident: ii.ident,
-                    ty: &const_.ty,
-                    mutability: ast::Mutability::Not,
-                    expr_opt: const_.expr.as_ref(),
-                    defaultness: Some(const_.defaultness),
-                    span: ii.span,
-            }
-            }
+            ast::AssocItemKind::Const(const_) => StaticParts {
+                prefix: "const",
+                vis: &ii.vis,
+                ident: ii.ident,
+                ty: &const_.ty,
+                mutability: ast::Mutability::Not,
+                expr_opt: const_.expr.as_ref(),
+                defaultness: Some(const_.defaultness),
+                span: ii.span,
             _ => unreachable!(),
         }
     }
test test::self_tests ... FAILED
---
---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
