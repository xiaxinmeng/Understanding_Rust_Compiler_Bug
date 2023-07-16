plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
failures:

---- compile_test stdout ----
normalized stderr:
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author.stage-id.aux"
------------------------------------------
------------------------------------------
if let StmtKind::Local(local) = stmt.kind
    && let Some(init) = local.init
    && let ExprKind::Cast(expr, cast_ty) = init.kind
    && let TyKind::Path(ref qpath) = cast_ty.kind
    && match_qpath(qpath, &["char"])
    && let ExprKind::Lit(ref lit) = expr.kind
    && let LitKind::Int(69, LitIntType::Unsuffixed) = lit.node
    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = local.pat.kind
    && name.as_str() == "x"
    // report your lint here
}

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph

------------------------------------------

normalized stderr:
normalized stderr:
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/call.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/call.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/call.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/call.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/call.stage-id.aux"
------------------------------------------
------------------------------------------
if let StmtKind::Local(local) = stmt.kind
    && let Some(init) = local.init
    && let ExprKind::Call(func, args) = init.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && match_qpath(qpath, &["{{root}}", "std", "cmp", "min"])
    && args.len() == 2
    && let ExprKind::Lit(ref lit) = args[0].kind
    && let LitKind::Int(3, LitIntType::Unsuffixed) = lit.node
    && let ExprKind::Lit(ref lit1) = args[1].kind
    && let LitKind::Int(4, LitIntType::Unsuffixed) = lit1.node
    && let PatKind::Wild = local.pat.kind
    // report your lint here
}

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph

------------------------------------------



error: failed to compile fixed code
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/as_underscore.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/as_underscore.stage-id" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "--crate-name=fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/as_underscore.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph

------------------------------------------

normalized stderr:
normalized stderr:
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/issue_3849.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/issue_3849.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/issue_3849.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/issue_3849.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/issue_3849.stage-id.aux"
------------------------------------------
------------------------------------------
if let StmtKind::Local(local) = stmt.kind
    && let Some(init) = local.init
    && let ExprKind::Call(func, args) = init.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && match_qpath(qpath, &["std", "mem", "transmute"])
    && args.len() == 1
    && let ExprKind::Path(ref qpath1) = args[0].kind
    && match_qpath(qpath1, &["ZPTR"])
    && let PatKind::Wild = local.pat.kind
    // report your lint here
}

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph

------------------------------------------

normalized stderr:
normalized stderr:
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/blocks.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/blocks.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/blocks.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/blocks.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/blocks.stage-id.aux"
------------------------------------------
------------------------------------------
if let ExprKind::Block(block, None) = expr.kind
    && block.stmts.len() == 3
    && let StmtKind::Local(local) = block.stmts[0].kind
    && let Some(init) = local.init
    && let ExprKind::Lit(ref lit) = init.kind
    && let LitKind::Int(42, LitIntType::Signed(IntTy::I32)) = lit.node
    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = local.pat.kind
    && name.as_str() == "x"
    && let StmtKind::Local(local1) = block.stmts[1].kind
    && let Some(init1) = local1.init
    && let ExprKind::Lit(ref lit1) = init1.kind
    && let LitKind::Float(_, LitFloatType::Suffixed(FloatTy::F32)) = lit1.node
    && let PatKind::Binding(BindingAnnotation::NONE, _, name1, None) = local1.pat.kind
    && name1.as_str() == "_t"
    && let StmtKind::Semi(e) = block.stmts[2].kind
    && let ExprKind::Unary(UnOp::Neg, inner) = e.kind
    && let ExprKind::Path(ref qpath) = inner.kind
    && match_qpath(qpath, &["x"])
    && block.expr.is_none()
    // report your lint here
}
}
if let ExprKind::Block(block, None) = expr.kind
    && block.stmts.len() == 1
    && let StmtKind::Local(local) = block.stmts[0].kind
    && let Some(init) = local.init
    && let ExprKind::Call(func, args) = init.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && match_qpath(qpath, &["String", "new"])
    && args.is_empty()
    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = local.pat.kind
    && name.as_str() == "expr"
    && let Some(trailing_expr) = block.expr
    && let ExprKind::Call(func1, args1) = trailing_expr.kind
    && let ExprKind::Path(ref qpath1) = func1.kind
    && match_qpath(qpath1, &["drop"])
    && args1.len() == 1
    && let ExprKind::Path(ref qpath2) = args1[0].kind
    && match_qpath(qpath2, &["expr"])
    // report your lint here
}
}
if let ExprKind::Closure(CaptureBy::Value, fn_decl, body_id, _, None) = expr.kind
    && let FnRetTy::DefaultReturn(_) = fn_decl.output
    && expr1 = &cx.tcx.hir().body(body_id).value
    && let ExprKind::Call(func, args) = expr1.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && matches!(qpath, QPath::LangItem(LangItem::IdentityFuture, _))
    && args.len() == 1
    && let ExprKind::Closure(CaptureBy::Value, fn_decl1, body_id1, _, Some(Movability::Static)) = args[0].kind
    && let FnRetTy::DefaultReturn(_) = fn_decl1.output
    && expr2 = &cx.tcx.hir().body(body_id1).value
    && let ExprKind::Block(block, None) = expr2.kind
    && block.stmts.is_empty()
    && block.expr.is_none()
    // report your lint here
}

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph

------------------------------------------

normalized stderr:
normalized stderr:
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/if.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/if.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/if.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/if.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/if.stage-id.aux"
------------------------------------------
------------------------------------------
if let StmtKind::Local(local) = stmt.kind
    && let Some(init) = local.init
    && let ExprKind::If(cond, then, Some(else_expr)) = init.kind
    && let ExprKind::DropTemps(expr) = cond.kind
    && let ExprKind::Lit(ref lit) = expr.kind
    && let LitKind::Bool(true) = lit.node
    && let ExprKind::Block(block, None) = then.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e) = block.stmts[0].kind
    && let ExprKind::Binary(op, left, right) = e.kind
    && BinOpKind::Eq == op.node
    && let ExprKind::Lit(ref lit1) = left.kind
    && let LitKind::Int(1, LitIntType::Unsuffixed) = lit1.node
    && let ExprKind::Lit(ref lit2) = right.kind
    && let LitKind::Int(1, LitIntType::Unsuffixed) = lit2.node
    && block.expr.is_none()
    && let ExprKind::Block(block1, None) = else_expr.kind
    && block1.stmts.len() == 1
    && let StmtKind::Semi(e1) = block1.stmts[0].kind
    && let ExprKind::Binary(op1, left1, right1) = e1.kind
    && BinOpKind::Eq == op1.node
    && let ExprKind::Lit(ref lit3) = left1.kind
    && let LitKind::Int(2, LitIntType::Unsuffixed) = lit3.node
    && let ExprKind::Lit(ref lit4) = right1.kind
    && let LitKind::Int(2, LitIntType::Unsuffixed) = lit4.node
    && block1.expr.is_none()
    && let PatKind::Wild = local.pat.kind
    // report your lint here
}
}
if let ExprKind::If(cond, then, Some(else_expr)) = expr.kind
    && let ExprKind::Let(let_expr) = cond.kind
    && let PatKind::Lit(lit_expr) = let_expr.pat.kind
    && let ExprKind::Lit(ref lit) = lit_expr.kind
    && let LitKind::Bool(true) = lit.node
    && let ExprKind::Path(ref qpath) = let_expr.init.kind
    && match_qpath(qpath, &["a"])
    && let ExprKind::Block(block, None) = then.kind
    && block.stmts.is_empty()
    && block.expr.is_none()
    && let ExprKind::Block(block1, None) = else_expr.kind
    && block1.stmts.is_empty()
    && block1.expr.is_none()
    // report your lint here
}

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph

------------------------------------------

normalized stderr:
normalized stderr:
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (9a274c72 2023-02-17)
query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/matches.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/matches.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/matches.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/matches.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/matches.stage-id.aux"
------------------------------------------
------------------------------------------
if let StmtKind::Local(local) = stmt.kind
    && let Some(init) = local.init
    && let ExprKind::Match(scrutinee, arms, MatchSource::Normal) = init.kind
    && let ExprKind::Lit(ref lit) = scrutinee.kind
    && let LitKind::Int(42, LitIntType::Unsuffixed) = lit.node
    && arms.len() == 3
    && let PatKind::Lit(lit_expr) = arms[0].pat.kind
    && let ExprKind::Lit(ref lit1) = lit_expr.kind
    && let LitKind::Int(16, LitIntType::Unsuffixed) = lit1.node
    && arms[0].guard.is_none()
    && let ExprKind::Lit(ref lit2) = arms[0].body.kind
    && let LitKind::Int(5, LitIntType::Unsuffixed) = lit2.node
    && let PatKind::Lit(lit_expr1) = arms[1].pat.kind
    && let ExprKind::Lit(ref lit3) = lit_expr1.kind
    && let LitKind::Int(17, LitIntType::Unsuffixed) = lit3.node
    && arms[1].guard.is_none()
    && let ExprKind::Block(block, None) = arms[1].body.kind
    && block.stmts.len() == 1
    && let StmtKind::Local(local1) = block.stmts[0].kind
    && let Some(init1) = local1.init
    && let ExprKind::Lit(ref lit4) = init1.kind
    && let LitKind::Int(3, LitIntType::Unsuffixed) = lit4.node
    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = local1.pat.kind
    && name.as_str() == "x"
    && let Some(trailing_expr) = block.expr
    && let ExprKind::Path(ref qpath) = trailing_expr.kind
    && match_qpath(qpath, &["x"])
    && let PatKind::Wild = arms[2].pat.kind
    && arms[2].guard.is_none()
    && let ExprKind::Lit(ref lit5) = arms[2].body.kind
    && let LitKind::Int(1, LitIntType::Unsuffixed) = lit5.node
    && let PatKind::Binding(BindingAnnotation::NONE, _, name1, None) = local.pat.kind
    && name1.as_str() == "a"
    // report your lint here
}

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1428:79

