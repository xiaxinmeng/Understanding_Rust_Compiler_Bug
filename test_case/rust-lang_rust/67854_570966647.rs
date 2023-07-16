plain
2020-01-06T00:08:53.7935593Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T00:08:53.7947705Z ##[command]git config gc.auto 0
2020-01-06T00:08:53.7950143Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T00:08:53.7952760Z ##[command]git config --get-all http.proxy
2020-01-06T00:08:53.7955131Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67854/merge:refs/remotes/pull/67854/merge
---
2020-01-06T00:45:55.6184290Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-01-06T00:45:55.9255064Z error: implementing `LintPass` by hand
2020-01-06T00:45:55.9256357Z    --> <::rustc::lint::declare_combined_early_lint_pass macros>:16:9
2020-01-06T00:45:55.9256858Z     |
2020-01-06T00:45:55.9257948Z 1   |   / ([$ v : vis $ name : ident, [$ ($ passes : ident : $ constructor : expr,) *]],
2020-01-06T00:45:55.9259046Z 2   |   |  $ methods : tt) =>
2020-01-06T00:45:55.9259616Z 3   |   | (# [allow (non_snake_case)] $ v struct $ name { $ ($ passes : $ passes,) * }
2020-01-06T00:45:55.9260131Z 4   |   |  impl $ name
2020-01-06T00:45:55.9260709Z ...     |
2020-01-06T00:45:55.9261362Z 16  |   |  } impl LintPass for $ name
2020-01-06T00:45:55.9262311Z     |   |         ^^^^^^^^
2020-01-06T00:45:55.9262914Z 17  |   |  { fn name (& self) -> & 'static str { panic ! () } })
2020-01-06T00:45:55.9263458Z     |   |______________________________________________________- in this expansion of `declare_combined_early_lint_pass!` (#4)
2020-01-06T00:45:55.9264155Z    ::: src/librustc_lint/lib.rs:75:1
2020-01-06T00:45:55.9264473Z     |
2020-01-06T00:45:55.9264473Z     |
2020-01-06T00:45:55.9264889Z 75  | /   macro_rules! pre_expansion_lint_passes {
2020-01-06T00:45:55.9265322Z 76  | |       ($macro:path, $args:tt) => {
2020-01-06T00:45:55.9265780Z 77  | |           $macro!($args, [KeywordIdents: KeywordIdents, UnusedDocComment: UnusedDocComment,]);
2020-01-06T00:45:55.9267250Z 78  | |       };
2020-01-06T00:45:55.9267674Z 79  | |   }
2020-01-06T00:45:55.9268117Z     | |___- in this expansion of `pre_expansion_lint_passes!` (#1)
2020-01-06T00:45:55.9268469Z ...
2020-01-06T00:45:55.9268469Z ...
2020-01-06T00:45:55.9268889Z 102 | /   macro_rules! declare_combined_early_pass {
2020-01-06T00:45:55.9269324Z 103 | |       ([$name:ident], $passes:tt) => (
2020-01-06T00:45:55.9270188Z 104 | |           early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
2020-01-06T00:45:55.9271282Z 105 | |       )
2020-01-06T00:45:55.9271675Z 106 | |   }
2020-01-06T00:45:55.9272131Z     | |___- in this expansion of `declare_combined_early_pass!` (#2)
2020-01-06T00:45:55.9272627Z 107 | 
2020-01-06T00:45:55.9272627Z 107 | 
2020-01-06T00:45:55.9273265Z 108 |     pre_expansion_lint_passes!(declare_combined_early_pass, [BuiltinCombinedPreExpansionLintPass]);
2020-01-06T00:45:55.9274450Z     | 
2020-01-06T00:45:55.9274828Z    ::: <::rustc::lint::early_lint_methods macros>:1:1
2020-01-06T00:45:55.9275178Z     |
2020-01-06T00:45:55.9275178Z     |
2020-01-06T00:45:55.9275602Z 1   |  /  ($ macro : path, $ args : tt) =>
2020-01-06T00:45:55.9276021Z 2   |  |  ($ macro !
2020-01-06T00:45:55.9276818Z 3   | ||   ($ args,
2020-01-06T00:45:55.9276818Z 3   | ||   ($ args,
2020-01-06T00:45:55.9277671Z 4   | ||    [fn check_param (a : & ast :: Param) ; fn check_ident (a : ast :: Ident) ;
2020-01-06T00:45:55.9278141Z 5   | ||     fn check_crate (a : & ast :: Crate) ; fn check_crate_post
2020-01-06T00:45:55.9278544Z ...   ||
2020-01-06T00:45:55.9279001Z 39  | ||     /// Counterpart to `enter_lint_attrs`.
2020-01-06T00:45:55.9279630Z 40  | ||      fn exit_lint_attrs (a : & [ast :: Attribute]) ;]) ;)
2020-01-06T00:45:55.9280090Z     | ||________________________________________________________-- in this expansion of `early_lint_methods!` (#3)
2020-01-06T00:45:55.9281862Z     |                                                           in this macro invocation (#4)
2020-01-06T00:45:55.9282085Z     |
2020-01-06T00:45:55.9282520Z     = note: `-D rustc::lint-pass-impl-without-macro` implied by `-D warnings`
2020-01-06T00:45:55.9282520Z     = note: `-D rustc::lint-pass-impl-without-macro` implied by `-D warnings`
2020-01-06T00:45:55.9282960Z     = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T00:45:55.9416402Z error: implementing `LintPass` by hand
2020-01-06T00:45:55.9416710Z    --> <::rustc::lint::declare_combined_early_lint_pass macros>:16:9
2020-01-06T00:45:55.9416944Z     |
2020-01-06T00:45:55.9416944Z     |
2020-01-06T00:45:55.9417425Z 1   |   / ([$ v : vis $ name : ident, [$ ($ passes : ident : $ constructor : expr,) *]],
2020-01-06T00:45:55.9417783Z 2   |   |  $ methods : tt) =>
2020-01-06T00:45:55.9418632Z 3   |   | (# [allow (non_snake_case)] $ v struct $ name { $ ($ passes : $ passes,) * }
2020-01-06T00:45:55.9419002Z 4   |   |  impl $ name
2020-01-06T00:45:55.9419228Z ...     |
2020-01-06T00:45:55.9419672Z 16  |   |  } impl LintPass for $ name
2020-01-06T00:45:55.9420100Z     |   |         ^^^^^^^^
2020-01-06T00:45:55.9420592Z 17  |   |  { fn name (& self) -> & 'static str { panic ! () } })
2020-01-06T00:45:55.9420928Z     |   |______________________________________________________- in this expansion of `declare_combined_early_lint_pass!` (#4)
2020-01-06T00:45:55.9421355Z    ::: src/librustc_lint/lib.rs:81:1
2020-01-06T00:45:55.9421535Z     |
2020-01-06T00:45:55.9421535Z     |
2020-01-06T00:45:55.9421811Z 81  | /   macro_rules! early_lint_passes {
2020-01-06T00:45:55.9422082Z 82  | |       ($macro:path, $args:tt) => {
2020-01-06T00:45:55.9422334Z 83  |             $macro!(
2020-01-06T00:45:55.9422770Z 84  |                 $args,
2020-01-06T00:45:55.9423021Z 85  |                 [
2020-01-06T00:45:55.9423269Z 86  |                     UnusedParens: UnusedParens,
2020-01-06T00:45:55.9423435Z ...
2020-01-06T00:45:55.9423435Z ...
2020-01-06T00:45:55.9423665Z 97  |                 ]
2020-01-06T00:45:55.9423901Z 98  | |           );
2020-01-06T00:45:55.9424165Z     | |____________- in this macro invocation (#2)
2020-01-06T00:45:55.9424427Z 99  | |       };
2020-01-06T00:45:55.9424657Z 100 | |   }
2020-01-06T00:45:55.9424929Z     | |___- in this expansion of `early_lint_passes!` (#1)
2020-01-06T00:45:55.9425151Z 101 | 
2020-01-06T00:45:55.9425412Z 102 | /   macro_rules! declare_combined_early_pass {
2020-01-06T00:45:55.9425676Z 103 | |       ([$name:ident], $passes:tt) => (
2020-01-06T00:45:55.9426245Z 104 | |           early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
2020-01-06T00:45:55.9427200Z 105 | |       )
2020-01-06T00:45:55.9427446Z 106 | |   }
2020-01-06T00:45:55.9427757Z     | |___- in this expansion of `declare_combined_early_pass!` (#2)
2020-01-06T00:45:55.9427943Z ...
2020-01-06T00:45:55.9427943Z ...
2020-01-06T00:45:55.9428238Z 109 |     early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
2020-01-06T00:45:55.9428965Z     | 
2020-01-06T00:45:55.9429359Z    ::: <::rustc::lint::early_lint_methods macros>:1:1
2020-01-06T00:45:55.9429555Z     |
2020-01-06T00:45:55.9429555Z     |
2020-01-06T00:45:55.9429812Z 1   |  /  ($ macro : path, $ args : tt) =>
2020-01-06T00:45:55.9430058Z 2   |  |  ($ macro !
2020-01-06T00:45:55.9430554Z 3   | ||   ($ args,
2020-01-06T00:45:55.9430554Z 3   | ||   ($ args,
2020-01-06T00:45:55.9430884Z 4   | ||    [fn check_param (a : & ast :: Param) ; fn check_ident (a : ast :: Ident) ;
2020-01-06T00:45:55.9431200Z 5   | ||     fn check_crate (a : & ast :: Crate) ; fn check_crate_post
2020-01-06T00:45:55.9431423Z ...   ||
2020-01-06T00:45:55.9431726Z 39  | ||     /// Counterpart to `enter_lint_attrs`.
2020-01-06T00:45:55.9432018Z 40  | ||      fn exit_lint_attrs (a : & [ast :: Attribute]) ;]) ;)
2020-01-06T00:45:55.9432355Z     | ||________________________________________________________-- in this expansion of `early_lint_methods!` (#3)
2020-01-06T00:45:55.9432886Z     |                                                           in this macro invocation (#4)
2020-01-06T00:45:55.9433087Z     |
2020-01-06T00:45:55.9433087Z     |
2020-01-06T00:45:55.9433327Z     = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T00:45:55.9463846Z error: implementing `LintPass` by hand
2020-01-06T00:45:55.9464230Z    --> <::rustc::lint::declare_combined_late_lint_pass macros>:14:7
2020-01-06T00:45:55.9464635Z     |
2020-01-06T00:45:55.9464635Z     |
2020-01-06T00:45:55.9464961Z 1   |   / ([$ v : vis $ name : ident, [$ ($ passes : ident : $ constructor : expr,) *]],
2020-01-06T00:45:55.9465345Z 2   |   |  [$ hir : tt], $ methods : tt) =>
2020-01-06T00:45:55.9465670Z 3   |   | (# [allow (non_snake_case)] $ v struct $ name { $ ($ passes : $ passes,) * }
2020-01-06T00:45:55.9465926Z 4   |   |  impl $ name
2020-01-06T00:45:55.9466124Z ...     |
2020-01-06T00:45:55.9466410Z 14  |   |  impl LintPass for $ name
2020-01-06T00:45:55.9466661Z     |   |       ^^^^^^^^
2020-01-06T00:45:55.9466943Z 15  |   |  { fn name (& self) -> & 'static str { panic ! () } })
2020-01-06T00:45:55.9467305Z     |   |______________________________________________________- in this expansion of `declare_combined_late_lint_pass!` (#4)
2020-01-06T00:45:55.9467714Z    ::: src/librustc_lint/lib.rs:111:1
2020-01-06T00:45:55.9467891Z     |
2020-01-06T00:45:55.9468153Z 111 | /   macro_rules! late_lint_passes {
2020-01-06T00:45:55.9468153Z 111 | /   macro_rules! late_lint_passes {
2020-01-06T00:45:55.9468435Z 112 | |       ($macro:path, $args:tt) => {
2020-01-06T00:45:55.9468665Z 113 |             $macro!(
2020-01-06T00:45:55.9469121Z 114 |                 $args,
2020-01-06T00:45:55.9469347Z 115 |                 [
2020-01-06T00:45:55.9469620Z 116 |                     // FIXME: Look into regression when this is used as a module lint
2020-01-06T00:45:55.9469815Z ...
2020-01-06T00:45:55.9469815Z ...
2020-01-06T00:45:55.9470034Z 132 |                 ]
2020-01-06T00:45:55.9470270Z 133 | |           );
2020-01-06T00:45:55.9470734Z     | |____________- in this macro invocation (#2)
2020-01-06T00:45:55.9470989Z 134 | |       };
2020-01-06T00:45:55.9471226Z 135 | |   }
2020-01-06T00:45:55.9471707Z     | |___- in this expansion of `late_lint_passes!` (#1)
2020-01-06T00:45:55.9471888Z ...
2020-01-06T00:45:55.9472654Z 170 | /   macro_rules! declare_combined_late_pass {
2020-01-06T00:45:55.9473195Z 171 | |       ([$v:vis $name:ident], $passes:tt) => (
2020-01-06T00:45:55.9473533Z 172 | |           late_lint_methods!(declare_combined_late_lint_pass, [$v $name, $passes], ['tcx]);
2020-01-06T00:45:55.9474453Z 173 | |       )
2020-01-06T00:45:55.9474715Z 174 | |   }
2020-01-06T00:45:55.9475020Z     | |___- in this expansion of `declare_combined_late_pass!` (#2)
2020-01-06T00:45:55.9475367Z ...
2020-01-06T00:45:55.9475367Z ...
2020-01-06T00:45:55.9475864Z 177 |     late_lint_passes!(declare_combined_late_pass, [pub BuiltinCombinedLateLintPass]);
2020-01-06T00:45:55.9477000Z     | 
2020-01-06T00:45:55.9477237Z    ::: <::rustc::lint::late_lint_methods macros>:1:1
2020-01-06T00:45:55.9477433Z     |
2020-01-06T00:45:55.9477433Z     |
2020-01-06T00:45:55.9477749Z 1   |  /  ($ macro : path, $ args : tt, [$ hir : tt]) =>
2020-01-06T00:45:55.9478028Z 2   |  |  ($ macro !
2020-01-06T00:45:55.9478285Z     |  |___-
2020-01-06T00:45:55.9478574Z 3   | ||   ($ args, [$ hir],
2020-01-06T00:45:55.9478923Z 4   | ||    [fn check_param (a : & $ hir hir :: Param < $ hir >) ; fn check_body
2020-01-06T00:45:55.9479278Z 5   | ||     (a : & $ hir hir :: Body < $ hir >) ; fn check_body_post
2020-01-06T00:45:55.9479683Z ...   ||
2020-01-06T00:45:55.9480015Z 51  | ||     /// Counterpart to `enter_lint_attrs`.
2020-01-06T00:45:55.9480325Z 52  | ||      fn exit_lint_attrs (a : & $ hir [ast :: Attribute]) ;]) ;)
2020-01-06T00:45:55.9480680Z     | ||______________________________________________________________-- in this expansion of `late_lint_methods!` (#3)
2020-01-06T00:45:55.9482424Z     |                                                                 in this macro invocation (#4)
2020-01-06T00:45:55.9482862Z     |
2020-01-06T00:45:55.9482862Z     |
2020-01-06T00:45:55.9483298Z     = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T00:45:55.9666553Z error: implementing `LintPass` by hand
2020-01-06T00:45:55.9667011Z    --> <::rustc::lint::declare_combined_late_lint_pass macros>:14:7
2020-01-06T00:45:55.9667236Z     |
2020-01-06T00:45:55.9667236Z     |
2020-01-06T00:45:55.9667761Z 1   |   / ([$ v : vis $ name : ident, [$ ($ passes : ident : $ constructor : expr,) *]],
2020-01-06T00:45:55.9668064Z 2   |   |  [$ hir : tt], $ methods : tt) =>
2020-01-06T00:45:55.9668427Z 3   |   | (# [allow (non_snake_case)] $ v struct $ name { $ ($ passes : $ passes,) * }
2020-01-06T00:45:55.9668891Z 4   |   |  impl $ name
2020-01-06T00:45:55.9669141Z ...     |
2020-01-06T00:45:55.9669433Z 14  |   |  impl LintPass for $ name
2020-01-06T00:45:55.9669721Z     |   |       ^^^^^^^^
2020-01-06T00:45:55.9670224Z 15  |   |  { fn name (& self) -> & 'static str { panic ! () } })
2020-01-06T00:45:55.9670800Z     |   |______________________________________________________- in this expansion of `declare_combined_late_lint_pass!` (#4)
2020-01-06T00:45:55.9671260Z    ::: src/librustc_lint/lib.rs:137:1
2020-01-06T00:45:55.9671452Z     |
2020-01-06T00:45:55.9671452Z     |
2020-01-06T00:45:55.9671749Z 137 | /   macro_rules! late_lint_mod_passes {
2020-01-06T00:45:55.9672195Z 138 | |       ($macro:path, $args:tt) => {
2020-01-06T00:45:55.9672451Z 139 |             $macro!(
2020-01-06T00:45:55.9672901Z 140 |                 $args,
2020-01-06T00:45:55.9673146Z 141 |                 [
2020-01-06T00:45:55.9673146Z 141 |                 [
2020-01-06T00:45:55.9673409Z 142 |                     HardwiredLints: HardwiredLints,
2020-01-06T00:45:55.9673997Z 165 |                 ]
2020-01-06T00:45:55.9674237Z 166 | |           );
2020-01-06T00:45:55.9674505Z     | |____________- in this macro invocation (#2)
2020-01-06T00:45:55.9674764Z 167 | |       };
2020-01-06T00:45:55.9674764Z 167 | |       };
2020-01-06T00:45:55.9675171Z 168 | |   }
2020-01-06T00:45:55.9675518Z     | |___- in this expansion of `late_lint_mod_passes!` (#1)
2020-01-06T00:45:55.9675775Z 169 | 
2020-01-06T00:45:55.9676031Z 170 | /   macro_rules! declare_combined_late_pass {
2020-01-06T00:45:55.9676294Z 171 | |       ([$v:vis $name:ident], $passes:tt) => (
2020-01-06T00:45:55.9677068Z 172 | |           late_lint_methods!(declare_combined_late_lint_pass, [$v $name, $passes], ['tcx]);
2020-01-06T00:45:55.9677727Z 173 | |       )
2020-01-06T00:45:55.9677974Z 174 | |   }
2020-01-06T00:45:55.9678273Z     | |___- in this expansion of `declare_combined_late_pass!` (#2)
2020-01-06T00:45:55.9678462Z ...
2020-01-06T00:45:55.9678462Z ...
2020-01-06T00:45:55.9678749Z 179 |     late_lint_mod_passes!(declare_combined_late_pass, [BuiltinCombinedModuleLateLintPass]);
2020-01-06T00:45:55.9679305Z     | 
2020-01-06T00:45:55.9679533Z    ::: <::rustc::lint::late_lint_methods macros>:1:1
2020-01-06T00:45:55.9679736Z     |
2020-01-06T00:45:55.9679736Z     |
2020-01-06T00:45:55.9680345Z 1   |  /  ($ macro : path, $ args : tt, [$ hir : tt]) =>
2020-01-06T00:45:55.9680792Z 2   |  |  ($ macro !
2020-01-06T00:45:55.9681199Z     |  |___-
2020-01-06T00:45:55.9681469Z 3   | ||   ($ args, [$ hir],
2020-01-06T00:45:55.9682187Z 4   | ||    [fn check_param (a : & $ hir hir :: Param < $ hir >) ; fn check_body
2020-01-06T00:45:55.9682837Z 5   | ||     (a : & $ hir hir :: Body < $ hir >) ; fn check_body_post
2020-01-06T00:45:55.9683137Z ...   ||
2020-01-06T00:45:55.9683836Z 51  | ||     /// Counterpart to `enter_lint_attrs`.
2020-01-06T00:45:55.9684346Z 52  | ||      fn exit_lint_attrs (a : & $ hir [ast :: Attribute]) ;]) ;)
2020-01-06T00:45:55.9684898Z     | ||______________________________________________________________-- in this expansion of `late_lint_methods!` (#3)
2020-01-06T00:45:55.9685956Z     |                                                                 in this macro invocation (#4)
2020-01-06T00:45:55.9686396Z     |
2020-01-06T00:45:55.9686396Z     |
2020-01-06T00:45:55.9686678Z     = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T00:45:57.0729456Z error: aborting due to 4 previous errors
2020-01-06T00:45:57.0732663Z 
2020-01-06T00:45:57.0824250Z error: could not compile `rustc_lint`.
2020-01-06T00:45:57.0840852Z warning: build failed, waiting for other jobs to finish...
---
2020-01-06T00:46:52.5817219Z   local time: Mon Jan  6 00:46:52 UTC 2020
2020-01-06T00:46:53.1081768Z   network time: Mon, 06 Jan 2020 00:46:53 GMT
2020-01-06T00:46:53.1085052Z == end clock drift check ==
2020-01-06T00:46:55.6282727Z 
2020-01-06T00:46:55.6378336Z ##[error]Bash exited with code '1'.
2020-01-06T00:46:55.6409925Z ##[section]Starting: Checkout
2020-01-06T00:46:55.6411368Z ==============================================================================
2020-01-06T00:46:55.6411410Z Task         : Get sources
2020-01-06T00:46:55.6411462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
