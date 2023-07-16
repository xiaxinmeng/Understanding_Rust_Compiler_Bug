
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.2 secs
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Copying stage2 std from stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage2 test from stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.2 secs
Copying stage2 rustc from stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 tool cargo (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Building stage2 tool rls (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
   Compiling clippy_lints v0.0.174 (file:///src/rustc-1.24.1-src/src/tools/clippy/clippy_lints)
error[E0407]: method `visit_lifetime_def` is not a member of trait `Visitor`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:411:5
    |
411 | /     fn visit_lifetime_def(&mut self, _: &'tcx LifetimeDef) {
412 | |         // don't actually visit `<'a>` or `<'a: 'b>`
413 | |         // we've already visited the `'a` declarations and
414 | |         // don't want to spuriously remove them
415 | |         // `'b` in `'a: 'b` is useless unless used elsewhere in
416 | |         // a non-lifetime bound
417 | |     }
    | |_____^ not a member of trait `Visitor`

error[E0433]: failed to resolve. Use of undeclared type or module `ViewPath_`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:40:17
   |
40 |                 ViewPath_::ViewPathSimple(ref name, ref path) => {
   |                 ^^^^^^^^^ Use of undeclared type or module `ViewPath_`

error[E0433]: failed to resolve. Use of undeclared type or module `ViewPath_`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:51:17
   |
51 |                 ViewPath_::ViewPathList(_, ref path_list_items) => for path_list_item in path_list_items.iter() {
   |                 ^^^^^^^^^ Use of undeclared type or module `ViewPath_`

error[E0433]: failed to resolve. Use of undeclared type or module `ViewPath_`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:57:17
   |
57 |                 ViewPath_::ViewPathGlob(_) => {},
   |                 ^^^^^^^^^ Use of undeclared type or module `ViewPath_`

error[E0531]: cannot find tuple struct/variant `TyImplTraitUniversal` in this scope
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:335:13
    |
335 |             TyImplTraitUniversal(_, ref param_bounds) => for bound in param_bounds {
    |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope

error[E0407]: method `visit_lifetime_def` is not a member of trait `Visitor`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:411:5
    |
411 | /     fn visit_lifetime_def(&mut self, _: &'tcx LifetimeDef) {
412 | |         // don't actually visit `<'a>` or `<'a: 'b>`
413 | |         // we've already visited the `'a` declarations and
414 | |         // don't want to spuriously remove them
415 | |         // `'b` in `'a: 'b` is useless unless used elsewhere in
416 | |         // a non-lifetime bound
417 | |     }
    | |_____^ not a member of trait `Visitor`

error[E0433]: failed to resolve. Use of undeclared type or module `ViewPath_`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:40:17
   |
40 |                 ViewPath_::ViewPathSimple(ref name, ref path) => {
   |                 ^^^^^^^^^ Use of undeclared type or module `ViewPath_`

error[E0433]: failed to resolve. Use of undeclared type or module `ViewPath_`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:51:17
   |
51 |                 ViewPath_::ViewPathList(_, ref path_list_items) => for path_list_item in path_list_items.iter() {
   |                 ^^^^^^^^^ Use of undeclared type or module `ViewPath_`

error[E0433]: failed to resolve. Use of undeclared type or module `ViewPath_`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:57:17
   |
57 |                 ViewPath_::ViewPathGlob(_) => {},
   |                 ^^^^^^^^^ Use of undeclared type or module `ViewPath_`

error[E0531]: cannot find tuple struct/variant `TyImplTraitUniversal` in this scope
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:335:13
    |
335 |             TyImplTraitUniversal(_, ref param_bounds) => for bound in param_bounds {
    |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope

error[E0615]: attempted to take value of method `ty_params` on type `&'tcx rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:104:26
    |
104 |     for typ in &generics.ty_params {
    |                          ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0615]: attempted to take value of method `lifetimes` on type `&'tcx rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:125:52
    |
125 |     if could_use_elision(cx, decl, body, &generics.lifetimes, bounds_lts) {
    |                                                    ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0615]: attempted to take value of method `ty_params` on type `&'tcx rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:104:26
    |
104 |     for typ in &generics.ty_params {
    |                          ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0615]: attempted to take value of method `lifetimes` on type `&'tcx rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:125:52
    |
125 |     if could_use_elision(cx, decl, body, &generics.lifetimes, bounds_lts) {
    |                                                    ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0609]: no field `bound_lifetimes` on type `&rustc::hir::WhereBoundPredicate`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:373:58
    |
373 |                 let allowed_lts = allowed_lts_from(&pred.bound_lifetimes);
    |                                                          ^^^^^^^^^^^^^^^

error[E0615]: attempted to take value of method `lifetimes` on type `&'tcx rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:425:10
    |
425 |         .lifetimes
    |          ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0609]: no field `bound_lifetimes` on type `&rustc::hir::WhereBoundPredicate`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:373:58
    |
373 |                 let allowed_lts = allowed_lts_from(&pred.bound_lifetimes);
    |                                                          ^^^^^^^^^^^^^^^

error[E0615]: attempted to take value of method `lifetimes` on type `&'tcx rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:425:10
    |
425 |         .lifetimes
    |          ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0615]: attempted to take value of method `ty_params` on type `&rustc::hir::Generics`
    --> src/tools/clippy/clippy_lints/src/methods.rs:1852:18
     |
1852 |         generics.ty_params.iter().any(|param| {
     |                  ^^^^^^^^^
     |
     = help: maybe a `()` to call it is missing?

error[E0609]: no field `ty_params` on type `&syntax::ast::Generics`
   --> src/tools/clippy/clippy_lints/src/misc_early.rs:191:24
    |
191 |         for ty in &gen.ty_params {
    |                        ^^^^^^^^^

error[E0615]: attempted to take value of method `ty_params` on type `&rustc::hir::Generics`
    --> src/tools/clippy/clippy_lints/src/methods.rs:1852:18
     |
1852 |         generics.ty_params.iter().any(|param| {
     |                  ^^^^^^^^^
     |
     = help: maybe a `()` to call it is missing?

error[E0609]: no field `ty_params` on type `&syntax::ast::Generics`
   --> src/tools/clippy/clippy_lints/src/misc_early.rs:191:24
    |
191 |         for ty in &gen.ty_params {
    |                        ^^^^^^^^^

error[E0615]: attempted to take value of method `ty_params` on type `rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/new_without_default.rs:106:48
    |
106 |                         if !impl_item.generics.ty_params.is_empty() {
    |                                                ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0615]: attempted to take value of method `ty_params` on type `rustc::hir::Generics`
   --> src/tools/clippy/clippy_lints/src/new_without_default.rs:106:48
    |
106 |                         if !impl_item.generics.ty_params.is_empty() {
    |                                                ^^^^^^^^^
    |
    = help: maybe a `()` to call it is missing?

error[E0609]: no field `bound_lifetimes` on type `&rustc::hir::PolyTraitRef`
   --> src/tools/clippy/clippy_lints/src/types.rs:958:41
    |
958 |                     .any(|bound| !bound.bound_lifetimes.is_empty());
    |                                         ^^^^^^^^^^^^^^^

error[E0609]: no field `bound_lifetimes` on type `&rustc::hir::PolyTraitRef`
   --> src/tools/clippy/clippy_lints/src/types.rs:958:41
    |
958 |                     .any(|bound| !bound.bound_lifetimes.is_empty());
    |                                         ^^^^^^^^^^^^^^^

error[E0609]: no field `node` on type `&syntax::ptr::P<syntax::ast::UseTree>`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:39:28
   |
39 |             match item_use.node {
   |                            ^^^^

error[E0609]: no field `node` on type `&syntax::ptr::P<syntax::ast::UseTree>`
  --> src/tools/clippy/clippy_lints/src/unsafe_removed_from_name.rs:39:28
   |
39 |             match item_use.node {
   |                            ^^^^

error: aborting due to 14 previous errors

error: Could not compile `clippy_lints`.
warning: build failed, waiting for other jobs to finish...
error: aborting due to 14 previous errors

error: Could not compile `clippy_lints`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/src/rustc-1.24.1-src/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--manifest-path" "/src/rustc-1.24.1-src/src/tools/clippy/Cargo.toml"
expected success, got: exit code: 101


Building stage2 tool rustfmt (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.1 secs
Building stage2 tool miri (x86_64-unknown-linux-gnu)
   Compiling miri v0.1.0 (file:///src/rustc-1.24.1-src/src/tools/miri)
error[E0464]: multiple matching crates for `log`
  --> src/tools/miri/miri/lib.rs:10:1
   |
10 | extern crate log;
   | ^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `log`: /src/rustc-1.24.1-src/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-693d476ed0b0eae3.rlib
           crate `log`: /src/rustc-1.24.1-src/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-a075f299b7e5eb19.rlib

error[E0463]: can't find crate for `log`
  --> src/tools/miri/miri/lib.rs:10:1
   |
10 | extern crate log;
   | ^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to 2 previous errors

error: Could not compile `miri`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/src/rustc-1.24.1-src/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--manifest-path" "/src/rustc-1.24.1-src/src/tools/miri/Cargo.toml"
expected success, got: exit code: 101


Build completed successfully in 0:00:38
