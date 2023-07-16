plain
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
    | 
   ::: /checkout/library/core/src/default.rs:167:1
    |
167 | / pub macro Default($item:item) {
169 | | }
    | |_- in this expansion of `#[derive(Default)]`
    |
    = note: cannot satisfy `_: std::default::Default`
    = note: cannot satisfy `_: std::default::Default`
    = note: required by `std::default::Default::default`

error[E0061]: this function takes 8 arguments but 7 arguments were supplied
   --> src/tools/rls/rls/src/project_model.rs:220:5
    |
220 |     ops::resolve_with_previous(registry, ws, &ResolveOpts::everything(), prev, None, &[], true)
    |     |
    |     expected 8 arguments
    |
note: function defined here
note: function defined here
   --> /checkout/src/tools/cargo/src/cargo/ops/resolve.rs:206:8
    |
206 | pub fn resolve_with_previous<'cfg>(

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0061, E0271, E0283, E0432, E0433, E0560.
---
Verifying status of rustfmt...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to build-fail during beta week.
{"book":"test-pass","rust-by-example":"test-pass","rls":"build-fail","rustbook":"test-fail","reference":"test-pass","nomicon":"test-pass","embedded-book":"test-pass","cargo-miri":"test-fail","rustfmt":"test-pass","miri":"test-pass","edition-guide":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
