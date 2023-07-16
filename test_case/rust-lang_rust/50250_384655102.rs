plain
[00:19:05]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:19:06] error[E0308]: mismatched types
[00:19:06]    --> librustc_traits/lowering.rs:224:77
[00:19:06]     |
[00:19:06] 224 |     let wellformed_wc = DomainGoal::WellFormed(WhereClauseAtom::Implemented(where_clauses));
[00:19:06]     |                                                                             ^^^^^^^^^^^^^ expected struct `rustc::ty::TraitPredicate`, found struct `std::vec::Vec`
[00:19:06]     |
[00:19:06]     = note: expected type `rustc::ty::TraitPredicate<'_>`
[00:19:06]                found type `std::vec::Vec<rustc::ty::Binder<rustc::traits::DomainGoal<'_>>>`
[00:19:06] 
[00:19:06] error[E0277]: the trait bound `rustc::traits::DomainGoal<'_>: std::iter::Iterator` is not satisfied
[00:19:06]    --> librustc_traits/lowering.rs:228:20
[00:19:06]     |
[00:19:06] 228 |     wellformed_wcs.extend(wellformed_wc);
[00:19:06]     |                    ^^^^^^ `rustc::traits::DomainGoal<'_>` is not an iterator; maybe try calling `.iter()` or a similar method
[00:19:06]     |
[00:19:06]     = help: the trait `std::iter::Iterator` is not implemented for `rustc::traits::DomainGoal<'_>`
[00:19:06]     = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `rustc::traits::DomainGoal<'_>`
[00:19:06] error[E0308]: mismatched types
[00:19:06]    --> librustc_traits/lowering.rs:268:55
[00:19:06]     |
[00:19:06]     |
[00:19:06] 268 |                 .map(|wc| Goal::from_poly_domain_goal(wc, tcx)),
[00:19:06]     |                                                       |
[00:19:06]     |                                                       |
[00:19:06]     |                                                       expected struct `rustc::ty::Binder`, found enum `rustc::traits::DomainGoal`
[00:19:06]     |                                                       help: try using a variant of the expected type: `rustc::ty::Binder(wc)`
[00:19:06]     |
[00:19:06]     = note: expected type `rustc::ty::Binder<rustc::traits::DomainGoal<'_>>`
[00:19:06]                found type `rustc::traits::DomainGoal<'_>`
[00:19:06] 
[00:19:06] error[E0277]: the trait bound `rustc::traits::ProgramClause<'_>: std::iter::Iterator` is not satisfied
[00:19:06]    --> librustc_traits/lowering.rs:275:14
[00:19:06]     |
[00:19:06] 275 |             .chain(wellformed_traitref),
[00:19:06]     |              ^^^^^ `rustc::traits::ProgramClause<'_>` is not an iterator; maybe try calling `.iter()` or a similar method
[00:19:06]     |
[00:19:06]     = help: the trait `std::iter::Iterator` is not implemented for `rustc::traits::ProgramClause<'_>`
[00:19:06]     = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `rustc::traits::ProgramClause<'_>`
[00:19:06] 
[00:19:06] error[E0277]: the trait bound `rustc::traits::ProgramClause<'_>: std::iter::Iterator` is not satisfied
[00:19:06]    --> librustc_traits/lowering.rs:272:9
[00:19:06]     |
[00:19:06] 272 |     tcx.mk_clauses(
[00:19:06]     |         ^^^^^^^^^^ `rustc::traits::ProgramClause<'_>` is not an iterator; maybe try calling `.iter()` or a similar method
[00:19:06]     |
[00:19:06]     = help: the trait `std::iter::Iterator` is not implemented for `rustc::traits::ProgramClause<'_>`
[00:19:06]     = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::iter::Chain<std::iter::Chain<std::iter::Once<rustc::traits::Clause<'_>>, std::iter::Map<std::slice::Iter<'_, rustc::ty::Predicate<'_>>, [closure@librustc_traits/lowering.rs:252:14: 252:64 tcx:_, trait_pred:_]>>, rustc::traits::ProgramClause<'_>>`
[00:19:06]     = note: required because of the requirements on the impl of `rustc::ty::context::InternAs<[rustc::traits::Clause<'_>], &rustc::ty::Slice<rustc::traits::Clause<'_>>>` for `std::iter::Chain<std::iter::Chain<std::iter::Once<rustc::traits::Clause<'_>>, std::iter::Map<std::slice::Iter<'_, rustc::ty::Predicate<'_>>, [closure@librustc_traits/lowering.rs:252:14: 252:64 tcx:_, trait_pred:_]>>, rustc::traits::ProgramClause<'_>>`
[00:19:06] error: aborting due to 5 previous errors
[00:19:06] 
[00:19:06] Some errors occurred: E0277, E0308.
[00:19:06] For more information about an error, try `rustc --explain E0277`.
[00:19:06] For more information about an error, try `rustc --explain E0277`.
[00:19:06] error: Could not compile `rustc_traits`.
[00:19:06] 
[00:19:06] Caused by:
[00:19:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_traits librustc_traits/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=f874c027ab685544 -C extra-filename=-f874c027ab685544 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3e9ac89279d9d9fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
[00:19:52] error: build failed
[00:19:52] error: build failed
[00:19:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:52] expected success, got: exit code: 101
[00:19:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:19:52] travis_fold:end:stage0-rustc

[00:19:52] travis_time:end:stage0-rustc:start=1524750987123813941,finish=1524751858652724865,duration=871528910924


[00:19:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:52] Build completed unsuccessfully in 0:14:45
[00:19:52] Makefile:28: recipe for target 'all' failed
[00:19:52] make: *** [all] Error 1
314856 ./src/llvm
256592 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
241188 ./src/llvm-emscripten
210060 ./src/llvm/test
---
149020 ./.git/modules/src
143664 ./obj/build/bootstrap/debug/incremental
135420 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
122708 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka
122704 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka/s-f0h8ucxal7-gp84ip-2f3cmz1knheyt
91276 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
91272 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
89696 ./src/llvm/test/CodeGen
70944 ./obj/build/x86_64-unknown-linux-gnu/native
