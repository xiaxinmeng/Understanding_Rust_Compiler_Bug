plain
2020-03-22T04:14:12.7322586Z ========================== Starting Command Output ===========================
2020-03-22T04:14:12.7328392Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1e7aca6b-dafe-4b02-b15a-86d141f859b6.sh
2020-03-22T04:14:12.7328919Z 
2020-03-22T04:14:12.7333954Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T04:14:12.7356081Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70245/merge to s
2020-03-22T04:14:12.7359763Z Task         : Get sources
2020-03-22T04:14:12.7360090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T04:14:12.7360406Z Version      : 1.0.0
2020-03-22T04:14:12.7360621Z Author       : Microsoft
---
2020-03-22T04:14:13.7244421Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T04:14:13.7250165Z ##[command]git config gc.auto 0
2020-03-22T04:14:13.7254226Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T04:14:13.7258236Z ##[command]git config --get-all http.proxy
2020-03-22T04:14:13.7264942Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70245/merge:refs/remotes/pull/70245/merge
---
2020-03-22T04:22:05.2634166Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-22T04:22:05.6160210Z error[E0532]: expected unit struct, unit variant or constant, found tuple variant `ty::Error`
2020-03-22T04:22:05.6161256Z    --> src/librustc_infer/infer/sub.rs:120:15
2020-03-22T04:22:05.6161869Z     |
2020-03-22T04:22:05.6162647Z 120 |             (&ty::Error, _) | (_, &ty::Error) => {
2020-03-22T04:22:05.6163787Z     |               ^^^^^^^^^ did you mean `ty::Error( /* fields */ )`?
2020-03-22T04:22:05.6165315Z help: possible better candidates are found in other modules, you can import them into scope
2020-03-22T04:22:05.6165990Z     |
2020-03-22T04:22:05.6166652Z 1   | use core::fmt::Error;
2020-03-22T04:22:05.6167545Z     |
2020-03-22T04:22:05.6167545Z     |
2020-03-22T04:22:05.6168450Z 1   | use crate::traits::project::ProjectionCacheEntry::Error;
2020-03-22T04:22:05.6169186Z     |
2020-03-22T04:22:05.6169870Z 1   | use log::Level::Error;
2020-03-22T04:22:05.6170480Z     |
2020-03-22T04:22:05.6171158Z 1   | use log::LevelFilter::Error;
2020-03-22T04:22:05.6172256Z       and 6 other candidates
2020-03-22T04:22:05.6172509Z 
2020-03-22T04:22:05.6181385Z error[E0532]: expected unit struct, unit variant or constant, found tuple variant `ty::Error`
2020-03-22T04:22:05.6182294Z    --> src/librustc_infer/infer/sub.rs:120:36
2020-03-22T04:22:05.6182294Z    --> src/librustc_infer/infer/sub.rs:120:36
2020-03-22T04:22:05.6182885Z     |
2020-03-22T04:22:05.6183688Z 120 |             (&ty::Error, _) | (_, &ty::Error) => {
2020-03-22T04:22:05.6184892Z     |                                    ^^^^^^^^^ did you mean `ty::Error( /* fields */ )`?
2020-03-22T04:22:05.6186493Z help: possible better candidates are found in other modules, you can import them into scope
2020-03-22T04:22:05.6187142Z     |
2020-03-22T04:22:05.6187823Z 1   | use core::fmt::Error;
2020-03-22T04:22:05.6188434Z     |
2020-03-22T04:22:05.6188434Z     |
2020-03-22T04:22:05.6189214Z 1   | use crate::traits::project::ProjectionCacheEntry::Error;
2020-03-22T04:22:05.6189970Z     |
2020-03-22T04:22:05.6190643Z 1   | use log::Level::Error;
2020-03-22T04:22:05.6191252Z     |
2020-03-22T04:22:05.6191953Z 1   | use log::LevelFilter::Error;
2020-03-22T04:22:05.6193043Z       and 6 other candidates
2020-03-22T04:22:05.6193230Z 
2020-03-22T04:22:06.1845916Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:06.1847086Z     --> src/librustc_infer/infer/mod.rs:1694:63
2020-03-22T04:22:06.1847086Z     --> src/librustc_infer/infer/mod.rs:1694:63
2020-03-22T04:22:06.1847743Z      |
2020-03-22T04:22:06.1849522Z 1694 |             values: Types(ExpectedFound { expected: tcx.types.err(), found: tcx.types.err() }),
2020-03-22T04:22:06.1851907Z      |                                                               ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:06.1878950Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:06.1880069Z     --> src/librustc_infer/infer/mod.rs:1694:87
2020-03-22T04:22:06.1881672Z      |
2020-03-22T04:22:06.1881672Z      |
2020-03-22T04:22:06.1882642Z 1694 |             values: Types(ExpectedFound { expected: tcx.types.err(), found: tcx.types.err() }),
2020-03-22T04:22:06.1884239Z      |                                                                                       ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:06.2229937Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:06.2231111Z    --> src/librustc_infer/infer/canonical/mod.rs:157:44
2020-03-22T04:22:06.2231747Z     |
2020-03-22T04:22:06.2231747Z     |
2020-03-22T04:22:06.2232643Z 157 |                         ty: self.tcx.types.err(), // FIXME(const_generics)
2020-03-22T04:22:06.2233977Z     |                                            ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:07.2160141Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:07.2161260Z    --> src/librustc_infer/infer/resolve.rs:193:38
2020-03-22T04:22:07.2161818Z     |
2020-03-22T04:22:07.2161818Z     |
2020-03-22T04:22:07.2162395Z 193 |                     self.tcx().types.err()
2020-03-22T04:22:07.2163378Z     |                                      ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:07.2194233Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:07.2197866Z    --> src/librustc_infer/infer/resolve.rs:197:38
2020-03-22T04:22:07.2198559Z     |
2020-03-22T04:22:07.2198559Z     |
2020-03-22T04:22:07.2199277Z 197 |                     self.tcx().types.err()
2020-03-22T04:22:07.2200266Z     |                                      ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:07.2257665Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:07.2258698Z    --> src/librustc_infer/infer/resolve.rs:201:38
2020-03-22T04:22:07.2259324Z     |
2020-03-22T04:22:07.2259324Z     |
2020-03-22T04:22:07.2260038Z 201 |                     self.tcx().types.err()
2020-03-22T04:22:07.2261266Z     |                                      ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:07.2282043Z error[E0609]: no field `consts` on type `rustc::ty::TyCtxt<'_>`
2020-03-22T04:22:07.2282981Z    --> src/librustc_infer/infer/resolve.rs:234:39
2020-03-22T04:22:07.2283598Z     |
2020-03-22T04:22:07.2283598Z     |
2020-03-22T04:22:07.2284335Z 234 |                     return self.tcx().consts.err;
2020-03-22T04:22:07.2336620Z 
2020-03-22T04:22:07.2522437Z error[E0599]: no method named `err` found for struct `rustc::ty::context::CommonTypes<'_>` in the current scope
2020-03-22T04:22:07.2523751Z    --> src/librustc_infer/infer/sub.rs:122:37
2020-03-22T04:22:07.2524354Z     |
2020-03-22T04:22:07.2524354Z     |
2020-03-22T04:22:07.2525220Z 122 |                 Ok(self.tcx().types.err())
2020-03-22T04:22:07.2526516Z     |                                     ^^^ method not found in `rustc::ty::context::CommonTypes<'_>`
2020-03-22T04:22:07.3645345Z error: aborting due to 10 previous errors
2020-03-22T04:22:07.3645816Z 
2020-03-22T04:22:07.3651249Z Some errors have detailed explanations: E0532, E0599, E0609.
2020-03-22T04:22:07.3655254Z For more information about an error, try `rustc --explain E0532`.
2020-03-22T04:22:07.3655254Z For more information about an error, try `rustc --explain E0532`.
2020-03-22T04:22:07.3738683Z error: could not compile `rustc_infer`.
2020-03-22T04:22:07.3745997Z 
2020-03-22T04:22:07.3747216Z To learn more, run the command again with --verbose.
2020-03-22T04:22:07.3756730Z warning: build failed, waiting for other jobs to finish...
2020-03-22T04:22:08.1485158Z error: build failed
2020-03-22T04:22:08.1514595Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-22T04:22:08.1529924Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-22T04:22:08.1530661Z Build completed unsuccessfully in 0:04:25
2020-03-22T04:22:08.1578496Z == clock drift check ==
2020-03-22T04:22:08.1593395Z   local time: Sun Mar 22 04:22:08 UTC 2020
2020-03-22T04:22:08.1593395Z   local time: Sun Mar 22 04:22:08 UTC 2020
2020-03-22T04:22:08.4494865Z   network time: Sun, 22 Mar 2020 04:22:08 GMT
2020-03-22T04:22:08.4498370Z == end clock drift check ==
2020-03-22T04:22:09.2548787Z 
2020-03-22T04:22:09.2626284Z ##[error]Bash exited with code '1'.
2020-03-22T04:22:09.2647660Z ##[section]Finishing: Run build
2020-03-22T04:22:09.2693943Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70245/merge to s
2020-03-22T04:22:09.2699331Z Task         : Get sources
2020-03-22T04:22:09.2699635Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T04:22:09.2699914Z Version      : 1.0.0
2020-03-22T04:22:09.2700201Z Author       : Microsoft
2020-03-22T04:22:09.2700201Z Author       : Microsoft
2020-03-22T04:22:09.2700521Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T04:22:09.2700876Z ==============================================================================
2020-03-22T04:22:09.6071255Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T04:22:09.6125109Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70245/merge to s
2020-03-22T04:22:09.6206139Z Cleaning up task key
2020-03-22T04:22:09.6207239Z Start cleaning up orphan processes.
2020-03-22T04:22:09.6378651Z Terminate orphan process: pid (3624) (python)
2020-03-22T04:22:09.6671988Z ##[section]Finishing: Finalize Job
