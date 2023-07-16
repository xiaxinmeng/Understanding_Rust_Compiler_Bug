plain
travis_time:end:1ccdb3aa:start=1544148674992513330,finish=1544148677675179971,duration=2682666641
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:49]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:15]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:23]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:23]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:24] error[E0432]: unresolved import `rustc::traits::TraitRefExpansionInfoDignosticBuilder`
[00:13:24]   --> src/librustc_typeck/astconv.rs:23:27
[00:13:24]    |
[00:13:24] 23 | use rustc::traits::{self, TraitRefExpansionInfoDignosticBuilder};
[00:13:24]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TraitRefExpansionInfoDignosticBuilder` in `traits`
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]   --> src/librustc_typeck/check/method/suggest.rs:77:51
[00:13:24]    |
[00:13:24]    |
[00:13:24] 77 |                                rcvr_expr: Option<&hir::Expr>,
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]   --> src/librustc_typeck/check/method/suggest.rs:79:52
[00:13:24]    |
[00:13:24]    |
[00:13:24] 79 |                                args: Option<&'gcx [hir::Expr]>) {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:244:29
[00:13:24]     |
[00:13:24]     |
[00:13:24] 244 |                             hir::ExprKind::Lit(ref lit) => {
[00:13:24]     |                             ^^^ Use of undeclared type or module `hir`
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:260:29
[00:13:24]     |
[00:13:24] 260 |                             hir::ExprKind::Path(ref qpath) => {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:262:41
[00:13:24]     |
[00:13:24]     |
[00:13:24] 262 |                                 if let &hir::QPath::Resolved(_, ref path) = &qpath {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:263:44
[00:13:24]     |
[00:13:24]     |
[00:13:24] 263 |                                     if let hir::def::Def::Local(node_id) = path.def {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:278:77
[00:13:24]     |
[00:13:24]     |
[00:13:24] 278 |                                             (FileName::Real(_), Node::Local(hir::Local {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:279:57
[00:13:24]     |
[00:13:24]     |
[00:13:24] 279 |                                                 source: hir::LocalSource::Normal,
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:393:39
[00:13:24]     |
[00:13:24]     |
[00:13:24] 393 |                         } else if let hir::ExprKind::Path(hir::QPath::Resolved(_, ref path)) =
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:393:59
[00:13:24]     |
[00:13:24]     |
[00:13:24] 393 |                         } else if let hir::ExprKind::Path(hir::QPath::Resolved(_, ref path)) =
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:587:52
[00:13:24]     |
[00:13:24]     |
[00:13:24] 587 |                                 rcvr_expr: Option<&hir::Expr>,
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:723:18
[00:13:24]     |
[00:13:24]     |
[00:13:24] 723 |         map: &'a hir::map::Map<'tcx>,
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:728:41
[00:13:24]     |
[00:13:24]     |
[00:13:24] 728 |         fn visit_item(&mut self, i: &'v hir::Item) {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:729:20
[00:13:24]     |
[00:13:24]     |
[00:13:24] 729 |             if let hir::ItemKind::Trait(..) = i.node {
[00:13:24]     |                    ^^^ Use of undeclared type or module `hir`
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:735:54
[00:13:24]     |
[00:13:24] 735 |         fn visit_trait_item(&mut self, _trait_item: &hir::TraitItem) {}
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:737:52
[00:13:24]     |
[00:13:24]     |
[00:13:24] 737 |         fn visit_impl_item(&mut self, _impl_item: &hir::ImplItem) {}
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:796:22
[00:13:24]     |
[00:13:24]     |
[00:13:24] 796 |         krate: &'tcx hir::Crate,
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:805:9
[00:13:24]     |
[00:13:24]     |
[00:13:24] 805 |         hir::intravisit::walk_crate(&mut finder, krate);
[00:13:24]     |         ^^^ Use of undeclared type or module `hir`
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:810:22
[00:13:24]     |
[00:13:24] 810 | impl<'a, 'tcx, 'gcx> hir::intravisit::Visitor<'tcx> for UsePlacementFinder<'a, 'tcx, 'gcx> {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:813:23
[00:13:24]     |
[00:13:24]     |
[00:13:24] 813 |         module: &'tcx hir::Mod,
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:821:13
[00:13:24]     |
[00:13:24]     |
[00:13:24] 821 |             hir::intravisit::walk_mod(self, module, node_id);
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:828:17
[00:13:24]     |
---
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:861:10
[00:13:24]     |
[00:13:24] 861 |     ) -> hir::intravisit::NestedVisitorMap<'this, 'tcx> {
[00:13:24] 
[00:13:24] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:862:9
[00:13:24]     |
---
[00:13:24] 
[00:13:24] error[E0425]: cannot find value `ty_string` in this scope
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:315:73
[00:13:24]     |
[00:13:24] 315 |                             err.note(&format!("did you mean `{}::{}`?", ty_string, suggestion));
[00:13:24]     |                                                                         ^^^^^^^^^ did you mean `ty_str`?
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `SelfSource` in this scope
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:654:37
[00:13:24]     |
[00:13:24] 654 |                             source: SelfSource) -> bool {
[00:13:24] 
[00:13:24] 
[00:13:24] error[E0425]: cannot find value `rcvr_expr` in this scope
[00:13:24]    --> src/librustc_typeck/check/method/suggest.rs:674:12
[00:13:24]     |
[00:13:24] 674 |         if rcvr_expr.is_none() {
[00:13:24] 
[00:13:24] 
[00:13:25] error: unused import: `TraitRefExpansionInfoDignosticBuilder`
[00:13:25]   --> src/librustc_typeck/astconv.rs:23:27
[00:13:25]    |
[00:13:25] 23 | use rustc::traits::{self, TraitRefExpansionInfoDignosticBuilder};
[00:13:25]    |
[00:13:25]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:25] 
[00:13:25] error: unused import: `std::ops::Range`
[00:13:25] error: unused import: `std::ops::Range`
[00:13:25]   --> src/librustc_typeck/astconv.rs:42:5
[00:13:25]    |
[00:13:25] 42 | use std::ops::Range;
[00:13:25]    |     ^^^^^^^^^^^^^^^
[00:13:25] 
[00:13:25] error: unused import: `rustc::hir::map as hir_map`
[00:13:25]   --> src/librustc_typeck/check/method/suggest.rs:22:5
[00:13:25]    |
[00:13:25] 22 | use rustc::hir::map as hir_map;
[00:13:25] 
[00:13:25] error: aborting due to 33 previous errors
[00:13:25] 
[00:13:25] Some errors occurred: E0412, E0425, E0432, E0433.
---
travis_time:end:1401d45d:start=1544149680007383371,finish=1544149680012857956,duration=5474585
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19bba0e4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:301c1f83
travis_time:start:301c1f83
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b846854
$ dmesg | grep -i kill
