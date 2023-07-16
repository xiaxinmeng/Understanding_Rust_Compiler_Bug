plain
2020-01-20T01:49:39.2218146Z ========================== Starting Command Output ===========================
2020-01-20T01:49:39.2219359Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d3c64151-7960-4122-bb77-21bebc4924ca.sh
2020-01-20T01:49:39.2219569Z 
2020-01-20T01:49:39.2221832Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-20T01:49:39.2226536Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68267/merge to s
2020-01-20T01:49:39.2228126Z Task         : Get sources
2020-01-20T01:49:39.2228150Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T01:49:39.2228206Z Version      : 1.0.0
2020-01-20T01:49:39.2228230Z Author       : Microsoft
---
2020-01-20T01:49:40.2018325Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-20T01:49:40.2027297Z ##[command]git config gc.auto 0
2020-01-20T01:49:40.2030285Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-20T01:49:40.2032947Z ##[command]git config --get-all http.proxy
2020-01-20T01:49:40.2038885Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68267/merge:refs/remotes/pull/68267/merge
---
2020-01-20T01:56:47.6074158Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-01-20T01:56:47.6656556Z error: expected identifier, found `,`
2020-01-20T01:56:47.6656896Z     --> src/librustc_resolve/lifetimes.rs:1324:56
2020-01-20T01:56:47.6657142Z      |
2020-01-20T01:56:47.6657677Z 1324 |         let LifetimeContext { tcx, map, lifetime_uses, , .. } = self;
2020-01-20T01:56:47.6682246Z 
2020-01-20T01:56:47.8442953Z error[E0432]: unresolved import `self::diagnostics`
2020-01-20T01:56:47.8443247Z   --> src/librustc_resolve/lifetimes.rs:30:11
2020-01-20T01:56:47.8443469Z    |
2020-01-20T01:56:47.8443469Z    |
2020-01-20T01:56:47.8443748Z 30 | use self::diagnostics::{add_missing_lifetime_specifiers_label, report_missing_lifetime_specifiers};
2020-01-20T01:56:47.8444112Z 
2020-01-20T01:56:47.8473748Z error: cannot find macro `pluralize` in this scope
2020-01-20T01:56:47.8474064Z     --> src/librustc_resolve/diagnostics.rs:1456:73
2020-01-20T01:56:47.8474459Z      |
2020-01-20T01:56:47.8474459Z      |
2020-01-20T01:56:47.8474752Z 1456 |     struct_span_err!(sess, span, E0106, "missing lifetime specifier{}", pluralize!(count))
2020-01-20T01:56:47.8543978Z 
2020-01-20T01:56:47.8750812Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2020-01-20T01:56:47.8751083Z     --> src/librustc_resolve/diagnostics.rs:1465:38
2020-01-20T01:56:47.8751292Z      |
2020-01-20T01:56:47.8751292Z      |
2020-01-20T01:56:47.8751566Z 1465 |     missing_named_lifetime_spots: &[&hir::Generics<'_>],
2020-01-20T01:56:47.8755555Z 
2020-01-20T01:56:47.9589706Z error[E0425]: cannot find value `tcx` in this scope
2020-01-20T01:56:47.9589992Z     --> src/librustc_resolve/lifetimes.rs:1329:19
2020-01-20T01:56:47.9590174Z      |
2020-01-20T01:56:47.9590174Z      |
2020-01-20T01:56:47.9590429Z 1329 |             tcx: *tcx,
2020-01-20T01:56:47.9590719Z      |                   ^^^ help: you might have meant to use the available field: `self.tcx`
2020-01-20T01:56:47.9637595Z error[E0425]: cannot find value `map` in this scope
2020-01-20T01:56:47.9638005Z     --> src/librustc_resolve/lifetimes.rs:1330:18
2020-01-20T01:56:47.9638427Z      |
2020-01-20T01:56:47.9638638Z 1330 |             map: map,
2020-01-20T01:56:47.9638638Z 1330 |             map: map,
2020-01-20T01:56:47.9638922Z      |                  ^^^ help: you might have meant to use the available field: `self.map`
2020-01-20T01:56:47.9682743Z error[E0425]: cannot find value `lifetime_uses` in this scope
2020-01-20T01:56:47.9682992Z     --> src/librustc_resolve/lifetimes.rs:1336:13
2020-01-20T01:56:47.9683192Z      |
2020-01-20T01:56:47.9683595Z 1336 |             lifetime_uses,
2020-01-20T01:56:47.9683595Z 1336 |             lifetime_uses,
2020-01-20T01:56:47.9683900Z      |             ^^^^^^^^^^^^^ help: you might have meant to use the available field: `self.lifetime_uses`
2020-01-20T01:56:48.0085637Z error: unused import: `rustc::session::Session`
2020-01-20T01:56:48.0085882Z   --> src/librustc_resolve/lifetimes.rs:11:5
2020-01-20T01:56:48.0086079Z    |
2020-01-20T01:56:48.0086291Z 11 | use rustc::session::Session;
2020-01-20T01:56:48.0086291Z 11 | use rustc::session::Session;
2020-01-20T01:56:48.0086498Z    |     ^^^^^^^^^^^^^^^^^^^^^^^
2020-01-20T01:56:48.0086682Z    |
2020-01-20T01:56:48.0086898Z    = note: `-D unused-imports` implied by `-D warnings`
2020-01-20T01:56:48.0090587Z 
2020-01-20T01:56:48.0115931Z error: unused import: `pluralize`
2020-01-20T01:56:48.0116179Z   --> src/librustc_resolve/lifetimes.rs:15:20
2020-01-20T01:56:48.0116349Z    |
2020-01-20T01:56:48.0116624Z 15 | use rustc_errors::{pluralize, struct_span_err, Applicability, DiagnosticBuilder};
2020-01-20T01:56:48.0119724Z 
2020-01-20T01:56:49.1920357Z error: aborting due to 9 previous errors
2020-01-20T01:56:49.1923765Z 
2020-01-20T01:56:49.1931228Z Some errors have detailed explanations: E0425, E0432, E0433.
2020-01-20T01:56:49.1931228Z Some errors have detailed explanations: E0425, E0432, E0433.
2020-01-20T01:56:49.1939291Z For more information about an error, try `rustc --explain E0425`.
2020-01-20T01:56:49.1999796Z error: could not compile `rustc_resolve`.
2020-01-20T01:56:49.2019515Z warning: build failed, waiting for other jobs to finish...
2020-01-20T01:57:12.8888649Z error: build failed
2020-01-20T01:57:12.8914076Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-20T01:57:12.8923547Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-20T01:57:12.8923776Z Build completed unsuccessfully in 0:05:12
2020-01-20T01:57:12.8971601Z == clock drift check ==
2020-01-20T01:57:12.8986975Z   local time: Mon Jan 20 01:57:12 UTC 2020
2020-01-20T01:57:12.8986975Z   local time: Mon Jan 20 01:57:12 UTC 2020
2020-01-20T01:57:13.1627465Z   network time: Mon, 20 Jan 2020 01:57:13 GMT
2020-01-20T01:57:13.1628266Z == end clock drift check ==
2020-01-20T01:57:13.5681094Z 
2020-01-20T01:57:13.5756753Z ##[error]Bash exited with code '1'.
2020-01-20T01:57:13.5773278Z ##[section]Finishing: Run build
2020-01-20T01:57:13.5787390Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68267/merge to s
2020-01-20T01:57:13.5789606Z Task         : Get sources
2020-01-20T01:57:13.5789642Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T01:57:13.5789679Z Version      : 1.0.0
2020-01-20T01:57:13.5789731Z Author       : Microsoft
2020-01-20T01:57:13.5789731Z Author       : Microsoft
2020-01-20T01:57:13.5789766Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-20T01:57:13.5789803Z ==============================================================================
2020-01-20T01:57:13.9295991Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-20T01:57:13.9330018Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68267/merge to s
2020-01-20T01:57:13.9430585Z Cleaning up task key
2020-01-20T01:57:13.9431268Z Start cleaning up orphan processes.
2020-01-20T01:57:13.9521911Z Terminate orphan process: pid (3529) (python)
2020-01-20T01:57:13.9689404Z ##[section]Finishing: Finalize Job
