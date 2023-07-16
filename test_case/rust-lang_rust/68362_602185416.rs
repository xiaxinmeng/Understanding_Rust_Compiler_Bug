plain
2020-03-22T11:25:40.9120278Z ========================== Starting Command Output ===========================
2020-03-22T11:25:40.9125621Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6646938c-2d53-4284-a8cc-60769e122fa6.sh
2020-03-22T11:25:40.9126324Z 
2020-03-22T11:25:40.9131683Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T11:25:40.9154694Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-22T11:25:40.9158309Z Task         : Get sources
2020-03-22T11:25:40.9158610Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T11:25:40.9158904Z Version      : 1.0.0
2020-03-22T11:25:40.9159104Z Author       : Microsoft
---
2020-03-22T11:25:41.9078162Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T11:25:41.9083889Z ##[command]git config gc.auto 0
2020-03-22T11:25:41.9087408Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T11:25:41.9091919Z ##[command]git config --get-all http.proxy
2020-03-22T11:25:41.9099536Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-03-22T11:33:29.1025899Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-22T11:33:31.4359929Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-22T11:33:32.2163827Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-22T11:33:33.9448638Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-22T11:33:43.8937672Z error[E0277]: the trait bound `ty::OutlivesPredicate<&ty::sty::RegionKind, &ty::sty::RegionKind>: ty::relate::Relate<'tcx>` is not satisfied
2020-03-22T11:33:43.8939203Z    --> src/librustc/ty/relate.rs:327:55
2020-03-22T11:33:43.8940552Z     |
2020-03-22T11:33:43.8941861Z 327 |             a.1.iter().zip(b.1).map(|(a, b)| relation.relate(a, b)),
2020-03-22T11:33:43.8944134Z     |                                                       ^^^^^^ the trait `ty::relate::Relate<'tcx>` is not implemented for `ty::OutlivesPredicate<&ty::sty::RegionKind, &ty::sty::RegionKind>`
2020-03-22T11:33:43.9725745Z error[E0308]: mismatched types
2020-03-22T11:33:43.9726470Z    --> src/librustc/ty/relate.rs:411:46
2020-03-22T11:33:43.9726986Z     |
2020-03-22T11:33:43.9726986Z     |
2020-03-22T11:33:43.9727751Z 411 |             let a_witness = GeneratorWitness(a_types,a_outlive_predicates);
2020-03-22T11:33:43.9728924Z     |                                              ^^^^^^^ expected reference, found struct `ty::sty::Binder`
2020-03-22T11:33:43.9729751Z     |
2020-03-22T11:33:43.9730724Z     = note: expected reference `&ty::List<&ty::TyS<'_>>`
2020-03-22T11:33:43.9731585Z                   found struct `ty::sty::Binder<&ty::List<&ty::TyS<'_>>>`
2020-03-22T11:33:44.0328996Z error[E0308]: mismatched types
2020-03-22T11:33:44.0329709Z    --> src/librustc/ty/relate.rs:411:54
2020-03-22T11:33:44.0330243Z     |
2020-03-22T11:33:44.0330243Z     |
2020-03-22T11:33:44.0331152Z 411 |             let a_witness = GeneratorWitness(a_types,a_outlive_predicates);
2020-03-22T11:33:44.0332937Z     |                                                      ^^^^^^^^^^^^^^^^^^^^ expected reference, found struct `ty::sty::Binder`
2020-03-22T11:33:44.0333848Z     |
2020-03-22T11:33:44.0334735Z     = note: expected reference `&ty::List<ty::OutlivesPredicate<&ty::sty::RegionKind, &ty::sty::RegionKind>>`
2020-03-22T11:33:44.0335888Z                   found struct `ty::sty::Binder<&ty::List<ty::OutlivesPredicate<&ty::sty::RegionKind, &ty::sty::RegionKind>>>`
2020-03-22T11:33:44.0911698Z error[E0308]: mismatched types
2020-03-22T11:33:44.0914777Z    --> src/librustc/ty/relate.rs:412:46
2020-03-22T11:33:44.0915685Z     |
2020-03-22T11:33:44.0915685Z     |
2020-03-22T11:33:44.0916651Z 412 |             let b_witness = GeneratorWitness(b_types,b_outlive_predicates);
2020-03-22T11:33:44.0918012Z     |                                              ^^^^^^^ expected reference, found struct `ty::sty::Binder`
2020-03-22T11:33:44.0919017Z     |
2020-03-22T11:33:44.0919936Z     = note: expected reference `&ty::List<&ty::TyS<'_>>`
2020-03-22T11:33:44.0920961Z                   found struct `ty::sty::Binder<&ty::List<&ty::TyS<'_>>>`
2020-03-22T11:33:44.1498918Z error[E0308]: mismatched types
2020-03-22T11:33:44.1500624Z    --> src/librustc/ty/relate.rs:412:54
2020-03-22T11:33:44.1501411Z     |
2020-03-22T11:33:44.1501411Z     |
2020-03-22T11:33:44.1502311Z 412 |             let b_witness = GeneratorWitness(b_types,b_outlive_predicates);
2020-03-22T11:33:44.1504013Z     |                                                      ^^^^^^^^^^^^^^^^^^^^ expected reference, found struct `ty::sty::Binder`
2020-03-22T11:33:44.1505005Z     |
2020-03-22T11:33:44.1506000Z     = note: expected reference `&ty::List<ty::OutlivesPredicate<&ty::sty::RegionKind, &ty::sty::RegionKind>>`
2020-03-22T11:33:44.1507732Z                   found struct `ty::sty::Binder<&ty::List<ty::OutlivesPredicate<&ty::sty::RegionKind, &ty::sty::RegionKind>>>`
2020-03-22T11:33:44.1508574Z 
2020-03-22T11:33:44.1548437Z error[E0599]: no method named `map_bound` found for struct `ty::relate::GeneratorWitness<'_>` in the current scope
2020-03-22T11:33:44.1549747Z    --> src/librustc/ty/relate.rs:414:66
2020-03-22T11:33:44.1552431Z     |
2020-03-22T11:33:44.1553336Z 312 | / struct GeneratorWitness<'tcx>(
2020-03-22T11:33:44.1555613Z 313 | |     &'tcx ty::List<Ty<'tcx>>,
2020-03-22T11:33:44.1556671Z 314 | |     &'tcx ty::List<ty::RegionOutlivesPredicate<'tcx>>,
2020-03-22T11:33:44.1557579Z 315 | | );
2020-03-22T11:33:44.1559040Z     | |__- method `map_bound` not found for this
2020-03-22T11:33:44.1559661Z ...
2020-03-22T11:33:44.1560408Z 414 |               let types = relation.relate(&a_witness, &b_witness)?.map_bound(|witness| witness.0);
2020-03-22T11:33:44.1561641Z     |                                                                    ^^^^^^^^^ method not found in `ty::relate::GeneratorWitness<'_>`
2020-03-22T11:33:44.1562253Z 
2020-03-22T11:33:44.1594012Z error[E0599]: no method named `map_bound` found for struct `ty::relate::GeneratorWitness<'_>` in the current scope
2020-03-22T11:33:44.1594893Z    --> src/librustc/ty/relate.rs:415:70
2020-03-22T11:33:44.1595419Z     |
2020-03-22T11:33:44.1596248Z 312 | / struct GeneratorWitness<'tcx>(
2020-03-22T11:33:44.1597258Z 313 | |     &'tcx ty::List<Ty<'tcx>>,
2020-03-22T11:33:44.1598532Z 314 | |     &'tcx ty::List<ty::RegionOutlivesPredicate<'tcx>>,
2020-03-22T11:33:44.1599623Z 315 | | );
2020-03-22T11:33:44.1600412Z     | |__- method `map_bound` not found for this
2020-03-22T11:33:44.1600949Z ...
2020-03-22T11:33:44.1601717Z 415 |               let predicates = relation.relate(&a_witness,&b_witness)?.map_bound(|witness|witness.1);
2020-03-22T11:33:44.1603241Z     |                                                                        ^^^^^^^^^ method not found in `ty::relate::GeneratorWitness<'_>`
2020-03-22T11:33:45.9484516Z error: aborting due to 7 previous errors
2020-03-22T11:33:45.9485378Z 
2020-03-22T11:33:45.9486249Z Some errors have detailed explanations: E0277, E0308, E0599.
2020-03-22T11:33:45.9487189Z For more information about an error, try `rustc --explain E0277`.
2020-03-22T11:33:45.9487189Z For more information about an error, try `rustc --explain E0277`.
2020-03-22T11:33:45.9792347Z error: could not compile `rustc`.
2020-03-22T11:33:45.9793114Z 
2020-03-22T11:33:45.9794501Z To learn more, run the command again with --verbose.
2020-03-22T11:33:45.9824292Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-22T11:33:45.9840083Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-22T11:33:45.9840747Z Build completed unsuccessfully in 0:04:18
2020-03-22T11:33:45.9901469Z == clock drift check ==
2020-03-22T11:33:45.9920978Z   local time: Sun Mar 22 11:33:45 UTC 2020
2020-03-22T11:33:45.9920978Z   local time: Sun Mar 22 11:33:45 UTC 2020
2020-03-22T11:33:46.2712357Z   network time: Sun, 22 Mar 2020 11:33:46 GMT
2020-03-22T11:33:46.2716999Z == end clock drift check ==
2020-03-22T11:33:46.9962573Z 
2020-03-22T11:33:47.0047586Z ##[error]Bash exited with code '1'.
2020-03-22T11:33:47.0062765Z ##[section]Finishing: Run build
2020-03-22T11:33:47.0113565Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-22T11:33:47.0118826Z Task         : Get sources
2020-03-22T11:33:47.0119184Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T11:33:47.0119509Z Version      : 1.0.0
2020-03-22T11:33:47.0119753Z Author       : Microsoft
2020-03-22T11:33:47.0119753Z Author       : Microsoft
2020-03-22T11:33:47.0120117Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T11:33:47.0120703Z ==============================================================================
2020-03-22T11:33:47.3810203Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T11:33:47.3859565Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-22T11:33:47.3955447Z Cleaning up task key
2020-03-22T11:33:47.3956703Z Start cleaning up orphan processes.
2020-03-22T11:33:47.4203889Z Terminate orphan process: pid (5674) (python)
2020-03-22T11:33:47.4432772Z ##[section]Finishing: Finalize Job
