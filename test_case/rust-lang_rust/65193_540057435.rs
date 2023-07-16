plain
2019-10-09T14:23:17.5019128Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T14:23:17.5181629Z ##[command]git config gc.auto 0
2019-10-09T14:23:17.5253775Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T14:23:17.5310707Z ##[command]git config --get-all http.proxy
2019-10-09T14:23:17.5437654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-09T14:31:19.9757355Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-09T14:31:21.3254353Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-09T14:31:22.5572820Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-09T14:31:34.6179665Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-09T14:31:47.3861598Z error[E0277]: `std::rc::Rc<lint::context::LintStore>` cannot be shared between threads safely
2019-10-09T14:31:47.3864809Z      |
2019-10-09T14:31:47.3864809Z      |
2019-10-09T14:31:47.3865012Z 1429 |     join(|| {
2019-10-09T14:31:47.3865317Z      |     ^^^^ `std::rc::Rc<lint::context::LintStore>` cannot be shared between threads safely
2019-10-09T14:31:47.3865491Z      | 
2019-10-09T14:31:47.3865757Z     ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.0/src/join/mod.rs:97:25
2019-10-09T14:31:47.3865932Z      |
2019-10-09T14:31:47.3866144Z 97   |     A: FnOnce() -> RA + Send,
2019-10-09T14:31:47.3867193Z      |                         ---- required by this bound in `rustc_rayon_core::join::join`
2019-10-09T14:31:47.3867435Z      |
2019-10-09T14:31:47.3868335Z      = help: within `ty::context::TyCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<lint::context::LintStore>`
2019-10-09T14:31:47.3869210Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-09T14:31:47.3869547Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-09T14:31:47.3869883Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-09T14:31:47.3870557Z      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
2019-10-09T14:31:47.3871008Z      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1429:10: 1434:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
2019-10-09T14:31:47.3871062Z 
2019-10-09T14:31:52.4173692Z error[E0277]: `std::rc::Rc<lint::context::LintStore>` cannot be shared between threads safely
2019-10-09T14:31:52.4175738Z      |
2019-10-09T14:31:52.4175738Z      |
2019-10-09T14:31:52.4177053Z 2786 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
2019-10-09T14:31:52.4178185Z      |                                                ^^^^^^^^ `std::rc::Rc<lint::context::LintStore>` cannot be shared between threads safely
2019-10-09T14:31:52.4178888Z      |
2019-10-09T14:31:52.4180706Z      = help: within `[closure@src/librustc/ty/mod.rs:2786:57: 2788:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<lint::context::LintStore>`
2019-10-09T14:31:52.4182040Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2019-10-09T14:31:52.4182595Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2019-10-09T14:31:52.4183285Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2019-10-09T14:31:52.4183733Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2019-10-09T14:31:52.4184242Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2786:57: 2788:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2019-10-09T14:31:52.4184565Z 
2019-10-09T14:31:56.7581499Z error[E0277]: `std::rc::Rc<lint::context::LintStore>` cannot be shared between threads safely
2019-10-09T14:31:56.7582744Z     --> src/librustc/ty/context.rs:1945:13
2019-10-09T14:31:56.7584828Z      |
2019-10-09T14:31:56.7585123Z 1945 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
2019-10-09T14:31:56.7585503Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<lint::context::LintStore>` cannot be shared between threads safely
2019-10-09T14:31:56.7590794Z     ::: /checkout/src/librustc_data_structures/sync.rs:427:32
2019-10-09T14:31:56.7591242Z      |
2019-10-09T14:31:56.7591242Z      |
2019-10-09T14:31:56.7591723Z 427  | pub fn assert_sync<T: ?Sized + Sync>() {}
2019-10-09T14:31:56.7592313Z      |                                ---- required by this bound in `rustc_data_structures::sync::assert_sync`
2019-10-09T14:31:56.7592749Z      |
2019-10-09T14:31:56.7593331Z      = help: within `ty::context::tls::ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<lint::context::LintStore>`
2019-10-09T14:31:56.7594149Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
2019-10-09T14:31:56.7594698Z      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
2019-10-09T14:31:56.7595202Z      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
2019-10-09T14:31:56.7596672Z      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
2019-10-09T14:31:58.0845307Z error: aborting due to 3 previous errors
2019-10-09T14:31:58.0845423Z 
2019-10-09T14:31:58.0846040Z For more information about this error, try `rustc --explain E0277`.
2019-10-09T14:31:58.2518775Z error: could not compile `rustc`.
---
2019-10-09T14:31:58.2602841Z == clock drift check ==
2019-10-09T14:31:58.2619949Z   local time: Wed Oct  9 14:31:58 UTC 2019
2019-10-09T14:31:58.3326637Z   network time: Wed, 09 Oct 2019 14:31:58 GMT
2019-10-09T14:31:58.3328567Z == end clock drift check ==
2019-10-09T14:31:59.0357566Z ##[error]Bash exited with code '1'.
2019-10-09T14:31:59.0387858Z ##[section]Starting: Checkout
2019-10-09T14:31:59.0389779Z ==============================================================================
2019-10-09T14:31:59.0389821Z Task         : Get sources
2019-10-09T14:31:59.0389874Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
