plain
2020-02-14T10:10:45.1719172Z ========================== Starting Command Output ===========================
2020-02-14T10:10:45.1722606Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f2356ff5-b454-43fb-8fc9-2d9d76afa741.sh
2020-02-14T10:10:45.1722785Z 
2020-02-14T10:10:45.1728655Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-14T10:10:45.1735116Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69109/merge to s
2020-02-14T10:10:45.1737120Z Task         : Get sources
2020-02-14T10:10:45.1737151Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T10:10:45.1737226Z Version      : 1.0.0
2020-02-14T10:10:45.1737257Z Author       : Microsoft
---
2020-02-14T10:10:46.1030968Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-14T10:10:46.1109597Z ##[command]git config gc.auto 0
2020-02-14T10:10:46.1184983Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-14T10:10:46.1241414Z ##[command]git config --get-all http.proxy
2020-02-14T10:10:46.1379364Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69109/merge:refs/remotes/pull/69109/merge
---
2020-02-14T10:18:46.1747159Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-02-14T10:18:46.9583616Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-14T10:18:48.6403485Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-14T10:19:00.6277593Z error[E0308]: mismatched types
2020-02-14T10:19:00.6278957Z    --> src/librustc/ty/query/plumbing.rs:163:17
2020-02-14T10:19:00.6280290Z 163 |                 key,
2020-02-14T10:19:00.6281017Z     |                 ^^^ expected associated type, found reference
2020-02-14T10:19:00.6281901Z     |
2020-02-14T10:19:00.6281901Z     |
2020-02-14T10:19:00.6282711Z     = note: expected associated type `<Q as ty::query::config::QueryConfig<'_>>::Key`
2020-02-14T10:19:00.6283380Z                      found reference `&<Q as ty::query::config::QueryConfig<'tcx>>::Key`
2020-02-14T10:19:00.6284458Z     = note: consider constraining the associated type `<Q as ty::query::config::QueryConfig<'_>>::Key` to `&<Q as ty::query::config::QueryConfig<'tcx>>::Key` or calling a method that returns `<Q as ty::query::config::QueryConfig<'_>>::Key`
2020-02-14T10:19:00.6287749Z 
2020-02-14T10:19:00.6287749Z 
2020-02-14T10:19:00.6310338Z error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
2020-02-14T10:19:00.6310718Z    --> src/librustc/ty/query/plumbing.rs:162:30
2020-02-14T10:19:00.6311053Z     |
2020-02-14T10:19:00.6311412Z 162 |             let cached = tcx.try_get_cached::<Q, _, _, _>(
2020-02-14T10:19:00.6311871Z     |                              ^^^^^^^^^^^^^^ expected closure that takes 2 distinct arguments
2020-02-14T10:19:00.6312203Z 163 |                 key,
2020-02-14T10:19:00.6312548Z 164 |                 |(value, index)| (value.clone(), index),
2020-02-14T10:19:00.6312978Z     |                 ---------------- takes a single 2-tuple as argument
2020-02-14T10:19:00.6313599Z help: change the closure to take multiple arguments instead of a single tuple
2020-02-14T10:19:00.6313852Z     |
2020-02-14T10:19:00.6313852Z     |
2020-02-14T10:19:00.6314204Z 164 |                 |value, index| (value.clone(), index),
2020-02-14T10:19:00.6314582Z 
2020-02-14T10:19:04.8549218Z error: aborting due to 2 previous errors
2020-02-14T10:19:04.8550070Z 
2020-02-14T10:19:04.8550741Z Some errors have detailed explanations: E0308, E0593.
2020-02-14T10:19:04.8550741Z Some errors have detailed explanations: E0308, E0593.
2020-02-14T10:19:04.8551471Z For more information about an error, try `rustc --explain E0308`.
2020-02-14T10:19:04.8861076Z error: could not compile `rustc`.
2020-02-14T10:19:04.8861838Z 
2020-02-14T10:19:04.8862807Z To learn more, run the command again with --verbose.
2020-02-14T10:19:04.8892529Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-14T10:19:04.8901264Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-14T10:19:04.8901568Z Build completed unsuccessfully in 0:05:18
2020-02-14T10:19:04.8956823Z == clock drift check ==
2020-02-14T10:19:04.8972286Z   local time: Fri Feb 14 10:19:04 UTC 2020
2020-02-14T10:19:04.8972286Z   local time: Fri Feb 14 10:19:04 UTC 2020
2020-02-14T10:19:05.1857372Z   network time: Fri, 14 Feb 2020 10:19:05 GMT
2020-02-14T10:19:05.1861217Z == end clock drift check ==
2020-02-14T10:19:05.8493105Z 
2020-02-14T10:19:05.8595228Z ##[error]Bash exited with code '1'.
2020-02-14T10:19:05.8607066Z ##[section]Finishing: Run build
2020-02-14T10:19:05.8623182Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69109/merge to s
2020-02-14T10:19:05.8625049Z Task         : Get sources
2020-02-14T10:19:05.8625106Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T10:19:05.8625146Z Version      : 1.0.0
2020-02-14T10:19:05.8625182Z Author       : Microsoft
2020-02-14T10:19:05.8625182Z Author       : Microsoft
2020-02-14T10:19:05.8625243Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-14T10:19:05.8625287Z ==============================================================================
2020-02-14T10:19:06.2803102Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-14T10:19:06.2846127Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69109/merge to s
2020-02-14T10:19:06.2968278Z Cleaning up task key
2020-02-14T10:19:06.2969097Z Start cleaning up orphan processes.
2020-02-14T10:19:06.3160983Z Terminate orphan process: pid (4556) (python)
2020-02-14T10:19:06.3365601Z ##[section]Finishing: Finalize Job
