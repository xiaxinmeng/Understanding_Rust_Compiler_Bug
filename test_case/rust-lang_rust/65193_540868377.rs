plain
2019-10-11T01:56:46.6534254Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T01:56:46.6637419Z ##[command]git config gc.auto 0
2019-10-11T01:56:46.6719712Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T01:56:46.6780216Z ##[command]git config --get-all http.proxy
2019-10-11T01:56:46.6946688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-11T02:03:59.7801013Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-11T02:04:01.2379528Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-11T02:04:02.4847816Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-11T02:04:15.0982589Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-11T02:04:28.2513359Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:28.2516118Z      |
2019-10-11T02:04:28.2516118Z      |
2019-10-11T02:04:28.2516665Z 1429 |     join(|| {
2019-10-11T02:04:28.2517422Z      |     ^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:28.2517916Z      | 
2019-10-11T02:04:28.2518530Z     ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.0/src/join/mod.rs:97:25
2019-10-11T02:04:28.2519013Z      |
2019-10-11T02:04:28.2519557Z 97   |     A: FnOnce() -> RA + Send,
2019-10-11T02:04:28.2520182Z      |                         ---- required by this bound in `rustc_rayon_core::join::join`
2019-10-11T02:04:28.2520892Z      |
2019-10-11T02:04:28.2521705Z      = help: the trait `std::marker::Send` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:28.2522841Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2523915Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2524746Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2525517Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2526267Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2526891Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:28.2527528Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:28.2528106Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2528710Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2529271Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2529909Z      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2530602Z      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1429:10: 1434:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
2019-10-11T02:04:28.2530817Z 
2019-10-11T02:04:28.2553820Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:28.2555378Z      |
2019-10-11T02:04:28.2555378Z      |
2019-10-11T02:04:28.2555928Z 1429 |     join(|| {
2019-10-11T02:04:28.2556698Z      |     ^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:28.2557368Z      | 
2019-10-11T02:04:28.2558077Z     ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.0/src/join/mod.rs:97:25
2019-10-11T02:04:28.2558576Z      |
2019-10-11T02:04:28.2559105Z 97   |     A: FnOnce() -> RA + Send,
2019-10-11T02:04:28.2559746Z      |                         ---- required by this bound in `rustc_rayon_core::join::join`
2019-10-11T02:04:28.2560498Z      |
2019-10-11T02:04:28.2561863Z      = help: the trait `std::marker::Send` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:28.2563158Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2564618Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2565252Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2565835Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2566365Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2566774Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:28.2567178Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:28.2567564Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2567929Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2568283Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2568683Z      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2569169Z      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1429:10: 1434:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
2019-10-11T02:04:28.2569240Z 
2019-10-11T02:04:28.2593189Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:28.2593888Z      |
2019-10-11T02:04:28.2593888Z      |
2019-10-11T02:04:28.2594182Z 1429 |     join(|| {
2019-10-11T02:04:28.2595091Z      |     ^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:28.2595659Z      | 
2019-10-11T02:04:28.2596020Z     ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.0/src/join/mod.rs:97:25
2019-10-11T02:04:28.2596290Z      |
2019-10-11T02:04:28.2596608Z 97   |     A: FnOnce() -> RA + Send,
2019-10-11T02:04:28.2597020Z      |                         ---- required by this bound in `rustc_rayon_core::join::join`
2019-10-11T02:04:28.2597290Z      |
2019-10-11T02:04:28.2597754Z      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:28.2598283Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2598803Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2599334Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2599890Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2600415Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2600792Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:28.2601193Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:28.2601574Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2601947Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2602426Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2602867Z      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2603338Z      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1429:10: 1434:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
2019-10-11T02:04:28.2603425Z 
2019-10-11T02:04:28.2610747Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:28.2611603Z      |
2019-10-11T02:04:28.2611603Z      |
2019-10-11T02:04:28.2612229Z 1429 |     join(|| {
2019-10-11T02:04:28.2612838Z      |     ^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:28.2613140Z      | 
2019-10-11T02:04:28.2613504Z     ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.0/src/join/mod.rs:97:25
2019-10-11T02:04:28.2613774Z      |
2019-10-11T02:04:28.2614107Z 97   |     A: FnOnce() -> RA + Send,
2019-10-11T02:04:28.2614514Z      |                         ---- required by this bound in `rustc_rayon_core::join::join`
2019-10-11T02:04:28.2614797Z      |
2019-10-11T02:04:28.2615286Z      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:28.2615853Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2616385Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:28.2616976Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2617520Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2618083Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:28.2618624Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:28.2619090Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:28.2619454Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2619838Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:28.2620335Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2620720Z      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:28.2621221Z      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1429:10: 1434:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
2019-10-11T02:04:28.2621275Z 
2019-10-11T02:04:33.3833054Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:33.3850222Z      |
2019-10-11T02:04:33.3850222Z      |
2019-10-11T02:04:33.3851391Z 2786 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
2019-10-11T02:04:33.3855030Z      |                                                ^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:33.3856028Z      |
2019-10-11T02:04:33.3859367Z      = help: the trait `std::marker::Send` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:33.3860383Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3861322Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3862279Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3863194Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3864716Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3865591Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:33.3866380Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:33.3867138Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3868787Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3869618Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3870222Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3870906Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2786:57: 2788:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2019-10-11T02:04:33.3871118Z 
2019-10-11T02:04:33.3871877Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:33.3872921Z      |
2019-10-11T02:04:33.3872921Z      |
2019-10-11T02:04:33.3874020Z 2786 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
2019-10-11T02:04:33.3875303Z      |                                                ^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:33.3875870Z      |
2019-10-11T02:04:33.3876582Z      = help: the trait `std::marker::Send` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:33.3877388Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3878771Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3879696Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3880500Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3881436Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3890275Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:33.3894326Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:33.3895295Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3895899Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3896514Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3897093Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3897749Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2786:57: 2788:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2019-10-11T02:04:33.3897965Z 
2019-10-11T02:04:33.3898676Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:33.3899711Z      |
2019-10-11T02:04:33.3899711Z      |
2019-10-11T02:04:33.3900284Z 2786 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
2019-10-11T02:04:33.3901086Z      |                                                ^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:33.3901593Z      |
2019-10-11T02:04:33.3902277Z      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:33.3903101Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3904214Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3905048Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3905843Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3906784Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3907465Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:33.3909727Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:33.3910360Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3918736Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3919575Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3920185Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3920855Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2786:57: 2788:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2019-10-11T02:04:33.3921068Z 
2019-10-11T02:04:33.3921693Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:33.3922712Z      |
2019-10-11T02:04:33.3922712Z      |
2019-10-11T02:04:33.3923579Z 2786 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
2019-10-11T02:04:33.3924655Z      |                                                ^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:33.3925173Z      |
2019-10-11T02:04:33.3925883Z      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:33.3926729Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3927467Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:33.3928265Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3929234Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3930070Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:33.3930696Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:33.3931521Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:33.3932133Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3932721Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-11T02:04:33.3933566Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3934274Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2019-10-11T02:04:33.3934949Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2786:57: 2788:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2019-10-11T02:04:33.3935186Z 
2019-10-11T02:04:37.7210355Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:37.7211577Z     --> src/librustc/ty/context.rs:1944:13
2019-10-11T02:04:37.7212333Z      |
2019-10-11T02:04:37.7213049Z 1944 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
2019-10-11T02:04:37.7214000Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:37.7215816Z     ::: /checkout/src/librustc_data_structures/sync.rs:427:32
2019-10-11T02:04:37.7216448Z      |
2019-10-11T02:04:37.7216448Z      |
2019-10-11T02:04:37.7217136Z 427  | pub fn assert_sync<T: ?Sized + Sync>() {}
2019-10-11T02:04:37.7217962Z      |                                ---- required by this bound in `rustc_data_structures::sync::assert_sync`
2019-10-11T02:04:37.7218593Z      |
2019-10-11T02:04:37.7219456Z      = help: the trait `std::marker::Send` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:37.7220338Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7221490Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7225378Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7225935Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7226739Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7227117Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:37.7227539Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:37.7227899Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7228252Z      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7228635Z      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
2019-10-11T02:04:37.7229013Z      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
2019-10-11T02:04:37.7229065Z 
2019-10-11T02:04:37.7243238Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:37.7243604Z     --> src/librustc/ty/context.rs:1944:13
2019-10-11T02:04:37.7243883Z      |
2019-10-11T02:04:37.7244221Z 1944 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
2019-10-11T02:04:37.7245150Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be sent between threads safely
2019-10-11T02:04:37.7245797Z     ::: /checkout/src/librustc_data_structures/sync.rs:427:32
2019-10-11T02:04:37.7246077Z      |
2019-10-11T02:04:37.7246077Z      |
2019-10-11T02:04:37.7246397Z 427  | pub fn assert_sync<T: ?Sized + Sync>() {}
2019-10-11T02:04:37.7246820Z      |                                ---- required by this bound in `rustc_data_structures::sync::assert_sync`
2019-10-11T02:04:37.7247108Z      |
2019-10-11T02:04:37.7247588Z      = help: the trait `std::marker::Send` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:37.7248353Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7248929Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7249672Z      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7250245Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7250779Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7251179Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:37.7251577Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:37.7251957Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7252314Z      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7252663Z      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
2019-10-11T02:04:37.7253048Z      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
2019-10-11T02:04:37.7253106Z 
2019-10-11T02:04:37.7263526Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:37.7263904Z     --> src/librustc/ty/context.rs:1944:13
2019-10-11T02:04:37.7264172Z      |
2019-10-11T02:04:37.7264897Z 1944 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
2019-10-11T02:04:37.7265555Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:37.7266185Z     ::: /checkout/src/librustc_data_structures/sync.rs:427:32
2019-10-11T02:04:37.7266439Z      |
2019-10-11T02:04:37.7266439Z      |
2019-10-11T02:04:37.7266775Z 427  | pub fn assert_sync<T: ?Sized + Sync>() {}
2019-10-11T02:04:37.7267377Z      |                                ---- required by this bound in `rustc_data_structures::sync::assert_sync`
2019-10-11T02:04:37.7267699Z      |
2019-10-11T02:04:37.7268186Z      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:37.7268698Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7269373Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7269919Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7270459Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7270986Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn lint::EarlyLintPass + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7271364Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:37.7271782Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:37.7272142Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7272494Z      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7272876Z      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
2019-10-11T02:04:37.7273243Z      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
2019-10-11T02:04:37.7273298Z 
2019-10-11T02:04:37.7273760Z error[E0277]: `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:37.7274066Z     --> src/librustc/ty/context.rs:1944:13
2019-10-11T02:04:37.7274309Z      |
2019-10-11T02:04:37.7275387Z 1944 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
2019-10-11T02:04:37.7276055Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)` cannot be shared between threads safely
2019-10-11T02:04:37.7276930Z     ::: /checkout/src/librustc_data_structures/sync.rs:427:32
2019-10-11T02:04:37.7277189Z      |
2019-10-11T02:04:37.7277189Z      |
2019-10-11T02:04:37.7277534Z 427  | pub fn assert_sync<T: ?Sized + Sync>() {}
2019-10-11T02:04:37.7277957Z      |                                ---- required by this bound in `rustc_data_structures::sync::assert_sync`
2019-10-11T02:04:37.7278399Z      |
2019-10-11T02:04:37.7278884Z      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)`
2019-10-11T02:04:37.7279454Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7279971Z      = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>`
2019-10-11T02:04:37.7280563Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7281113Z      = note: required because it appears within the type `alloc::raw_vec::RawVec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7281662Z      = note: required because it appears within the type `std::vec::Vec<std::boxed::Box<(dyn std::ops::Fn() -> std::boxed::Box<(dyn for<'a, 'tcx> lint::LateLintPass<'a, 'tcx> + std::marker::Send + std::marker::Sync + 'static)> + 'static)>>`
2019-10-11T02:04:37.7282062Z      = note: required because it appears within the type `lint::context::LintStore`
2019-10-11T02:04:37.7282463Z      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<lint::context::LintStore>`
2019-10-11T02:04:37.7282828Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7283204Z      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
2019-10-11T02:04:37.7283559Z      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
2019-10-11T02:04:37.7283943Z      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
2019-10-11T02:04:39.0468302Z error: aborting due to 12 previous errors
2019-10-11T02:04:39.0468413Z 
2019-10-11T02:04:39.0468762Z For more information about this error, try `rustc --explain E0277`.
2019-10-11T02:04:39.2081071Z error: could not compile `rustc`.
---
2019-10-11T02:04:39.2180666Z == clock drift check ==
2019-10-11T02:04:39.2200324Z   local time: Fri Oct 11 02:04:39 UTC 2019
2019-10-11T02:04:39.3700028Z   network time: Fri, 11 Oct 2019 02:04:39 GMT
2019-10-11T02:04:39.3703056Z == end clock drift check ==
2019-10-11T02:04:39.7393069Z ##[error]Bash exited with code '1'.
2019-10-11T02:04:39.7569454Z ##[section]Starting: Checkout
2019-10-11T02:04:39.7571377Z ==============================================================================
2019-10-11T02:04:39.7571438Z Task         : Get sources
2019-10-11T02:04:39.7571491Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
