plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
   Compiling url v2.1.1
   Compiling clippy_utils v0.1.52 (/checkout/src/tools/clippy/clippy_utils)
   Compiling cargo_metadata v0.12.0
   Compiling clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error: irrefutable `if let` pattern
    |
    |
131 |         if let panic_message = snippet_opt(cx, args[0].span);
    |
    |
    = note: `-D irrefutable-let-patterns` implied by `-D warnings`
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
373 |         if let tool_name = meta_item.path.segments[0].ident;
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
280 |             if let limit = &args[0];
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   --> src/tools/clippy/clippy_lints/src/loops/needless_collect.rs:100:24
    |
100 |                 if let ty = cx.typeck_results().node_type(ty.hir_id);
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   |
   |
54 |             if let s = path_lit.as_str();
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   |
   |
55 |             if let pushed_path = Path::new(&*s);
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   |
   |
44 |             if let snippet = snippet_with_macro_callsite(cx, expr.span, "}");
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
  --> src/tools/clippy/clippy_lints/src/single_component_path_imports.rs:46:20
   |
46 |             if let segments = &use_tree.prefix.segments;
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
  --> src/tools/clippy/clippy_lints/src/to_digit_is_some.rs:59:36
   |
59 | ...                   if let to_digits_call_res = cx.qpath_res(to_digits_path, to_digits_call.hir_id);
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
176 |         if let name = name_ident.ident.name.to_ident_string();
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
180 |         if let closure_body = cx.tcx.hir().body(*closure_body_id);
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`
error: aborting due to 11 previous errors

error: could not compile `clippy_lints`

---
   Compiling parity-tokio-ipc v0.8.0
   Compiling rustc-ap-rustc_data_structures v705.0.0
   Compiling jsonrpc-server-utils v17.0.0
   Compiling jsonrpc-pubsub v17.0.0
warning: irrefutable `if let` pattern
    |
    |
131 |         if let panic_message = snippet_opt(cx, args[0].span);
    |
    = note: `#[warn(irrefutable_let_patterns)]` on by default
    = note: `#[warn(irrefutable_let_patterns)]` on by default
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
    |
    |
373 |         if let tool_name = meta_item.path.segments[0].ident;
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
    |
    |
280 |             if let limit = &args[0];
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
   --> src/tools/clippy/clippy_lints/src/loops/needless_collect.rs:100:24
    |
100 |                 if let ty = cx.typeck_results().node_type(ty.hir_id);
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
   |
   |
54 |             if let s = path_lit.as_str();
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
   |
   |
55 |             if let pushed_path = Path::new(&*s);
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
   |
   |
44 |             if let snippet = snippet_with_macro_callsite(cx, expr.span, "}");
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
  --> src/tools/clippy/clippy_lints/src/single_component_path_imports.rs:46:20
   |
46 |             if let segments = &use_tree.prefix.segments;
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
  --> src/tools/clippy/clippy_lints/src/to_digit_is_some.rs:59:36
   |
59 | ...                   if let to_digits_call_res = cx.qpath_res(to_digits_path, to_digits_call.hir_id);
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
    |
    |
176 |         if let name = name_ident.ident.name.to_ident_string();
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

warning: irrefutable `if let` pattern
    |
    |
180 |         if let closure_body = cx.tcx.hir().body(*closure_body_id);
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`
   Compiling jsonrpc-ipc-server v17.0.1
   Compiling jsonrpc-client-transports v17.0.0
   Compiling rustc-ap-rustc_span v705.0.0
   Compiling jsonrpc-core-client v17.0.0
---
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
   Compiling cargo_metadata v0.12.0
   Compiling clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error: irrefutable `if let` pattern
    |
    |
131 |         if let panic_message = snippet_opt(cx, args[0].span);
    |
    |
    = note: `-D irrefutable-let-patterns` implied by `-D warnings`
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
373 |         if let tool_name = meta_item.path.segments[0].ident;
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
280 |             if let limit = &args[0];
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   --> src/tools/clippy/clippy_lints/src/loops/needless_collect.rs:100:24
    |
100 |                 if let ty = cx.typeck_results().node_type(ty.hir_id);
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   |
   |
54 |             if let s = path_lit.as_str();
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   |
   |
55 |             if let pushed_path = Path::new(&*s);
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
   |
   |
44 |             if let snippet = snippet_with_macro_callsite(cx, expr.span, "}");
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
  --> src/tools/clippy/clippy_lints/src/single_component_path_imports.rs:46:20
   |
46 |             if let segments = &use_tree.prefix.segments;
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
  --> src/tools/clippy/clippy_lints/src/to_digit_is_some.rs:59:36
   |
59 | ...                   if let to_digits_call_res = cx.qpath_res(to_digits_path, to_digits_call.hir_id);
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
176 |         if let name = name_ident.ident.name.to_ident_string();
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`

error: irrefutable `if let` pattern
    |
    |
180 |         if let closure_body = cx.tcx.hir().body(*closure_body_id);
    |
    |
    = note: this pattern will always match, so the `if let` is useless
    = help: consider replacing the `if let` with a `let`
error: aborting due to 11 previous errors

error: could not compile `clippy_lints`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:533:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/clippy
Build completed unsuccessfully in 0:00:14
