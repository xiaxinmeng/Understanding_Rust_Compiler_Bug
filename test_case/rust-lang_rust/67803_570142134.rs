plain
2020-01-02T07:59:29.4482193Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-02T07:59:29.4691813Z ##[command]git config gc.auto 0
2020-01-02T07:59:29.4758057Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-02T07:59:29.4835654Z ##[command]git config --get-all http.proxy
2020-01-02T07:59:29.4991485Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67803/merge:refs/remotes/pull/67803/merge
---
2020-01-02T08:06:52.4533135Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-01-02T08:06:53.8066702Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-01-02T08:06:54.2964516Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-01-02T08:06:55.3941570Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-01-02T08:07:03.2332307Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-01-02T08:07:03.5381459Z error: cannot find macro `parallel` in this scope
2020-01-02T08:07:03.5381845Z    --> <::rustc_data_structures::sync::parallel macros>:8:3
2020-01-02T08:07:03.5382115Z     |
2020-01-02T08:07:03.5382115Z     |
2020-01-02T08:07:03.5382514Z 1   | / (impl $ fblock : tt [$ ($ c : tt,) *] [$ block : tt $ (, $ rest : tt) *]) =>
2020-01-02T08:07:03.5382898Z 2   | | { parallel ! (impl $ fblock [$ block, $ ($ c,) *] [$ ($ rest), *]) } ;
2020-01-02T08:07:03.5383265Z 3   | | (impl $ fblock : tt [$ ($ blocks : tt,) *] []) =>
2020-01-02T08:07:03.5383550Z 4   | | {
2020-01-02T08:07:03.5383818Z ...   |
2020-01-02T08:07:03.5384416Z 7   | | } ; ($ fblock : tt, $ ($ blocks : tt), *) =>
2020-01-02T08:07:03.5384829Z 8   | | { parallel ! (impl $ fblock [] [$ ($ blocks), *]) ; } ;
2020-01-02T08:07:03.5385296Z     | |___^^^^^^^^____________________________________________- in this expansion of `rustc_data_structures::parallel!`
2020-01-02T08:07:03.5385533Z     | 
2020-01-02T08:07:03.5385809Z    ::: src/librustc_hir/hir.rs:670:9
2020-01-02T08:07:03.5386354Z 670 | /         rustc_data_structures::parallel!(
2020-01-02T08:07:03.5386674Z 671 | |             {
2020-01-02T08:07:03.5386674Z 671 | |             {
2020-01-02T08:07:03.5387024Z 672 | |                 par_for_each_in(&self.items, |(_, item)| {
2020-01-02T08:07:03.5387380Z 673 | |                     visitor.visit_item(item);
2020-01-02T08:07:03.5387920Z 685 | |             }
2020-01-02T08:07:03.5388228Z 686 | |         );
2020-01-02T08:07:03.5388537Z     | |__________- in this macro invocation
2020-01-02T08:07:03.5388578Z 
2020-01-02T08:07:03.5388578Z 
2020-01-02T08:07:03.7812369Z error: unused import: `par_for_each_in`
2020-01-02T08:07:03.7812726Z   --> src/librustc_hir/hir.rs:12:35
2020-01-02T08:07:03.7813166Z    |
2020-01-02T08:07:03.7813563Z 12 | use rustc_data_structures::sync::{par_for_each_in, Send, Sync};
2020-01-02T08:07:03.7814454Z    |
2020-01-02T08:07:03.7815224Z    = note: `-D unused-imports` implied by `-D warnings`
2020-01-02T08:07:03.7815272Z 
2020-01-02T08:07:05.5514812Z error: aborting due to 2 previous errors
2020-01-02T08:07:05.5514812Z error: aborting due to 2 previous errors
2020-01-02T08:07:05.5515318Z 
2020-01-02T08:07:05.5582684Z error: could not compile `rustc_hir`.
2020-01-02T08:07:05.5606809Z warning: build failed, waiting for other jobs to finish...
2020-01-02T08:07:05.7598527Z error: build failed
2020-01-02T08:07:05.7664675Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-02T08:07:05.7674437Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-02T08:07:05.7674586Z Build completed unsuccessfully in 0:04:17
2020-01-02T08:07:05.7725626Z == clock drift check ==
2020-01-02T08:07:05.7747858Z   local time: Thu Jan  2 08:07:05 UTC 2020
2020-01-02T08:07:05.7747858Z   local time: Thu Jan  2 08:07:05 UTC 2020
2020-01-02T08:07:06.0550968Z   network time: Thu, 02 Jan 2020 08:07:06 GMT
2020-01-02T08:07:06.0551116Z == end clock drift check ==
2020-01-02T08:07:07.3620412Z 
2020-01-02T08:07:07.3737422Z ##[error]Bash exited with code '1'.
2020-01-02T08:07:07.3770269Z ##[section]Starting: Checkout
2020-01-02T08:07:07.3772139Z ==============================================================================
2020-01-02T08:07:07.3772216Z Task         : Get sources
2020-01-02T08:07:07.3772270Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
