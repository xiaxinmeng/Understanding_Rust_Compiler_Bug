plain
2020-01-18T21:59:01.3720054Z ========================== Starting Command Output ===========================
2020-01-18T21:59:01.3721530Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b5848c64-994c-4538-96f5-8e54493052d9.sh
2020-01-18T21:59:01.6772409Z 
2020-01-18T21:59:01.6848102Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T21:59:01.6859548Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68350/merge to s
2020-01-18T21:59:01.6861688Z Task         : Get sources
2020-01-18T21:59:01.6861722Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T21:59:01.6861796Z Version      : 1.0.0
2020-01-18T21:59:01.6861826Z Author       : Microsoft
---
2020-01-18T21:59:07.3685828Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T21:59:07.3701377Z ##[command]git config gc.auto 0
2020-01-18T21:59:07.3705722Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T21:59:07.3709104Z ##[command]git config --get-all http.proxy
2020-01-18T21:59:07.3718992Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68350/merge:refs/remotes/pull/68350/merge
---
2020-01-18T22:31:51.5332689Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-01-18T22:32:15.5072168Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T22:32:15.5072565Z     --> src/librustc/ty/context.rs:205:13
2020-01-18T22:32:15.5072804Z      |
2020-01-18T22:32:15.5073124Z 205  | /             ty::tls::with(|tcx| {
2020-01-18T22:32:15.5073463Z 206  | |                 bug!(
2020-01-18T22:32:15.5073829Z 207  | |                     "node {} with HirId::owner {:?} cannot be placed in \
2020-01-18T22:32:15.5074356Z 208  | |                         TypeckTables with local_id_root {:?}",
2020-01-18T22:32:15.5074884Z 212  | |                 )
2020-01-18T22:32:15.5075147Z 213  | |             });
2020-01-18T22:32:15.5075374Z      | |______________^
2020-01-18T22:32:15.5075588Z      |
2020-01-18T22:32:15.5075588Z      |
2020-01-18T22:32:15.5075820Z note: the type parameter R here was inferred to `!`
2020-01-18T22:32:15.5076053Z     --> src/librustc/ty/context.rs:205:13
2020-01-18T22:32:15.5076427Z      |
2020-01-18T22:32:15.5076669Z 205  |             ty::tls::with(|tcx| {
2020-01-18T22:32:15.5077449Z note: (type parameter defined here)
2020-01-18T22:32:15.5077676Z     --> src/librustc/ty/context.rs:1792:20
2020-01-18T22:32:15.5077861Z      |
2020-01-18T22:32:15.5077861Z      |
2020-01-18T22:32:15.5078115Z 1792 |     pub fn with<F, R>(f: F) -> R
2020-01-18T22:32:15.5078568Z note: ... due to this expression evaluating to `!`
2020-01-18T22:32:15.5078804Z     --> src/librustc/macros.rs:42:9
2020-01-18T22:32:15.5078990Z      |
2020-01-18T22:32:15.5079241Z 39   | / macro_rules! bug {
2020-01-18T22:32:15.5079241Z 39   | / macro_rules! bug {
2020-01-18T22:32:15.5079550Z 40   | |     () => ( bug!("impossible case reached") );
2020-01-18T22:32:15.5079828Z 41   | |     ($($message:tt)*) => ({
2020-01-18T22:32:15.5080290Z 42   | |         $crate::util::bug::bug_fmt(file!(), line!(), format_args!($($message)*))
2020-01-18T22:32:15.5081360Z 43   | |     })
2020-01-18T22:32:15.5081676Z 44   | | }
2020-01-18T22:32:15.5081991Z      | |_- in this expansion of `bug!`
2020-01-18T22:32:15.5082229Z      | 
2020-01-18T22:32:15.5082229Z      | 
2020-01-18T22:32:15.5082512Z     ::: src/librustc/ty/context.rs:206:17
2020-01-18T22:32:15.5082729Z      |
2020-01-18T22:32:15.5083029Z 206  | /                 bug!(
2020-01-18T22:32:15.5083412Z 207  | |                     "node {} with HirId::owner {:?} cannot be placed in \
2020-01-18T22:32:15.5083769Z 208  | |                         TypeckTables with local_id_root {:?}",
2020-01-18T22:32:15.5084144Z 209  | |                     tcx.hir().node_to_string(hir_id),
2020-01-18T22:32:15.5084615Z 210  | |                     DefId::local(hir_id.owner),
2020-01-18T22:32:15.5085073Z 211  | |                     local_id_root
2020-01-18T22:32:15.5085619Z      | |_________________- in this macro invocation
2020-01-18T22:32:15.5085921Z      = note: If you want the `!` type to be used here, add explicit type annotations
2020-01-18T22:32:15.5085959Z 
2020-01-18T22:32:17.0270058Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T22:32:17.0270058Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T22:32:17.0271644Z     --> src/librustc/util/bug.rs:32:5
2020-01-18T22:32:17.0272440Z      |
2020-01-18T22:32:17.0273067Z 32   | /     tls::with_opt(move |tcx| {
2020-01-18T22:32:17.0274450Z 33   | |         let msg = format!("{}:{}: {}", file, line, args);
2020-01-18T22:32:17.0275102Z 34   | |         match (tcx, span) {
2020-01-18T22:32:17.0275681Z 35   | |             (Some(tcx), Some(span)) => tcx.sess.diagnostic().span_bug(span, &msg),
2020-01-18T22:32:17.0276651Z 38   | |         }
2020-01-18T22:32:17.0277139Z 39   | |     });
2020-01-18T22:32:17.0277577Z      | |______^
2020-01-18T22:32:17.0277968Z      |
2020-01-18T22:32:17.0277968Z      |
2020-01-18T22:32:17.0278387Z note: the type parameter R here was inferred to `!`
2020-01-18T22:32:17.0278985Z     --> src/librustc/util/bug.rs:32:5
2020-01-18T22:32:17.0279564Z      |
2020-01-18T22:32:17.0279993Z 32   |     tls::with_opt(move |tcx| {
2020-01-18T22:32:17.0281264Z note: (type parameter defined here)
2020-01-18T22:32:17.0281777Z     --> src/librustc/ty/context.rs:1802:24
2020-01-18T22:32:17.0282175Z      |
2020-01-18T22:32:17.0282175Z      |
2020-01-18T22:32:17.0282615Z 1802 |     pub fn with_opt<F, R>(f: F) -> R
2020-01-18T22:32:17.0283490Z note: ... due to this expression evaluating to `!`
2020-01-18T22:32:17.0283924Z     --> src/librustc/util/bug.rs:35:40
2020-01-18T22:32:17.0284429Z      |
2020-01-18T22:32:17.0284429Z      |
2020-01-18T22:32:17.0284844Z 35   |             (Some(tcx), Some(span)) => tcx.sess.diagnostic().span_bug(span, &msg),
2020-01-18T22:32:17.0285731Z      = note: If you want the `!` type to be used here, add explicit type annotations
2020-01-18T22:32:17.0285911Z 
2020-01-18T22:32:17.4198203Z error: aborting due to 2 previous errors
2020-01-18T22:32:17.4198868Z 
---
2020-01-18T22:32:32.9018920Z   local time: Sat Jan 18 22:32:32 UTC 2020
2020-01-18T22:32:33.1994634Z   network time: Sat, 18 Jan 2020 22:32:33 GMT
2020-01-18T22:32:33.2000132Z == end clock drift check ==
2020-01-18T22:32:33.6757138Z 
2020-01-18T22:32:33.6863321Z ##[error]Bash exited with code '1'.
2020-01-18T22:32:33.6875987Z ##[section]Finishing: Run build
2020-01-18T22:32:33.6929118Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68350/merge to s
2020-01-18T22:32:33.6931056Z Task         : Get sources
2020-01-18T22:32:33.6931098Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T22:32:33.6931154Z Version      : 1.0.0
2020-01-18T22:32:33.6931210Z Author       : Microsoft
2020-01-18T22:32:33.6931210Z Author       : Microsoft
2020-01-18T22:32:33.6931252Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T22:32:33.6931297Z ==============================================================================
2020-01-18T22:32:34.1205600Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T22:32:34.1247953Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68350/merge to s
2020-01-18T22:32:34.1368080Z Cleaning up task key
2020-01-18T22:32:34.1368788Z Start cleaning up orphan processes.
2020-01-18T22:32:34.1485242Z Terminate orphan process: pid (4274) (python)
2020-01-18T22:32:34.2103013Z ##[section]Finishing: Finalize Job
