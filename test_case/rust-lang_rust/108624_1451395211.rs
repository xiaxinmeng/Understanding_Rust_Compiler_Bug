plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
-             ty: (),
-             temp_lifetime: Some(
-                 Node(2),
-             ),
-             span: $DIR/thir-flat.rs:4:15: 4:17 (#0),
25             kind: Block {
26                 block: b0,

-         },
-         Expr {
30             ty: (),
30             ty: (),
31             temp_lifetime: Some(
32                 Node(2),

33             ),
34             span: $DIR/thir-flat.rs:4:15: 4:17 (#0),
+         Expr {
35             kind: Scope {
36                 region_scope: Node(2),
37                 lint_level: Explicit(
---
45             temp_lifetime: Some(
46                 Node(2),

47             ),
48             span: $DIR/thir-flat.rs:4:15: 4:17 (#0),
+         Expr {
49             kind: Scope {
50                 region_scope: Destruction(2),
51                 lint_level: Inherited,
51                 lint_level: Inherited,

52                 value: e1,
53             },
+             ty: (),
+             temp_lifetime: Some(
+                 Node(2),
+             ),
+             span: $DIR/thir-flat.rs:4:15: 4:17 (#0),
55     ],
56     stmts: [],



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-print/thir-flat/thir-flat.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args thir-print/thir-flat.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/thir-print/thir-flat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-print/thir-flat" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-print/thir-flat/auxiliary" "-Z" "unpretty=thir-flat"
--- stdout -------------------------------
DefId(0:3 ~ thir_flat[45a6]::main):
Thir {
    body_type: Fn(
        ([]; c_variadic: false)->(),
    arms: [],
    blocks: [
        Block {
        Block {
            targeted_by_break: false,
            region_scope: Node(1),
            opt_destruction_scope: None,
            span: fake-test-src-base/thir-print/thir-flat.rs:4:15: 4:17 (#0),
            stmts: [],
            expr: None,
            safety_mode: Safe,
    ],
    exprs: [
        Expr {
            kind: Block {
            kind: Block {
                block: b0,
            },
            ty: (),
            temp_lifetime: Some(
                Node(2),
            ),
            span: fake-test-src-base/thir-print/thir-flat.rs:4:15: 4:17 (#0),
        Expr {
            kind: Scope {
                region_scope: Node(2),
                lint_level: Explicit(
                lint_level: Explicit(
                    HirId(DefId(0:3 ~ thir_flat[45a6]::main).2),
                value: e0,
            },
            ty: (),
            temp_lifetime: Some(
            temp_lifetime: Some(
                Node(2),
            ),
            span: fake-test-src-base/thir-print/thir-flat.rs:4:15: 4:17 (#0),
        Expr {
            kind: Scope {
                region_scope: Destruction(2),
                lint_level: Inherited,
                lint_level: Inherited,
                value: e1,
            },
            ty: (),
            temp_lifetime: Some(
                Node(2),
            ),
            span: fake-test-src-base/thir-print/thir-flat.rs:4:15: 4:17 (#0),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
    ],
    stmts: [],
    params: [],
}
