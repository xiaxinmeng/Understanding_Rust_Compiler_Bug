plain
2020-02-19T04:18:29.9530792Z ========================== Starting Command Output ===========================
2020-02-19T04:18:29.9532193Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8ba4c014-f5f9-4a3f-9059-0e6fe23d9c02.sh
2020-02-19T04:18:29.9532224Z 
2020-02-19T04:18:29.9534879Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-19T04:18:29.9540933Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-02-19T04:18:29.9542524Z Task         : Get sources
2020-02-19T04:18:29.9542560Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T04:18:29.9542641Z Version      : 1.0.0
2020-02-19T04:18:29.9542676Z Author       : Microsoft
---
2020-02-19T04:18:30.9564562Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-19T04:18:30.9578296Z ##[command]git config gc.auto 0
2020-02-19T04:18:30.9581401Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-19T04:18:30.9583544Z ##[command]git config --get-all http.proxy
2020-02-19T04:18:30.9595516Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-02-19T04:25:34.4481421Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-19T04:25:36.6536096Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-02-19T04:25:37.3618525Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-19T04:25:38.7798551Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-19T04:25:39.4013180Z error[E0412]: cannot find type `RegionOutlivesPredicate` in this scope
2020-02-19T04:25:39.4013568Z    --> src/librustc/ty/sty.rs:214:40
2020-02-19T04:25:39.4013838Z     |
2020-02-19T04:25:39.4014185Z 214 |     GeneratorWitness(Binder<&'tcx List<RegionOutlivesPredicate<'tcx>>, &'tcx List<Ty<'tcx>>>),
2020-02-19T04:25:39.4014806Z     |
2020-02-19T04:25:39.4015095Z help: possible candidate is found in another module, you can import it into scope
2020-02-19T04:25:39.4015507Z     |
2020-02-19T04:25:39.4015507Z     |
2020-02-19T04:25:39.4015810Z 5   | use crate::ty::RegionOutlivesPredicate;
2020-02-19T04:25:39.4016367Z help: you might be missing a type parameter
2020-02-19T04:25:39.4016616Z     |
2020-02-19T04:25:39.4016616Z     |
2020-02-19T04:25:39.4016906Z 135 | pub enum TyKind<'tcx, RegionOutlivesPredicate> {
2020-02-19T04:25:39.4021457Z 
2020-02-19T04:25:40.5836155Z error[E0107]: wrong number of type arguments: expected 1, found 2
2020-02-19T04:25:40.5836617Z    --> src/librustc/ty/sty.rs:214:72
2020-02-19T04:25:40.5836953Z     |
2020-02-19T04:25:40.5836953Z     |
2020-02-19T04:25:40.5837409Z 214 |     GeneratorWitness(Binder<&'tcx List<RegionOutlivesPredicate<'tcx>>, &'tcx List<Ty<'tcx>>>),
2020-02-19T04:25:40.5842912Z 
2020-02-19T04:25:40.8044913Z error: aborting due to 2 previous errors
2020-02-19T04:25:40.8048863Z 
2020-02-19T04:25:40.8049360Z Some errors have detailed explanations: E0107, E0412.
2020-02-19T04:25:40.8049360Z Some errors have detailed explanations: E0107, E0412.
2020-02-19T04:25:40.8049655Z For more information about an error, try `rustc --explain E0107`.
2020-02-19T04:25:40.8205036Z error: could not compile `rustc`.
2020-02-19T04:25:40.8205137Z 
2020-02-19T04:25:40.8205386Z To learn more, run the command again with --verbose.
2020-02-19T04:25:40.8244435Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-19T04:25:40.8257604Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-19T04:25:40.8257687Z Build completed unsuccessfully in 0:04:36
2020-02-19T04:25:40.8322025Z == clock drift check ==
2020-02-19T04:25:40.8339028Z   local time: Wed Feb 19 04:25:40 UTC 2020
2020-02-19T04:25:40.8339028Z   local time: Wed Feb 19 04:25:40 UTC 2020
2020-02-19T04:25:41.0975495Z   network time: Wed, 19 Feb 2020 04:25:41 GMT
2020-02-19T04:25:41.0975644Z == end clock drift check ==
2020-02-19T04:25:41.9054181Z 
2020-02-19T04:25:41.9114773Z ##[error]Bash exited with code '1'.
2020-02-19T04:25:41.9125886Z ##[section]Finishing: Run build
2020-02-19T04:25:41.9143174Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-02-19T04:25:41.9144901Z Task         : Get sources
2020-02-19T04:25:41.9144954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T04:25:41.9145019Z Version      : 1.0.0
2020-02-19T04:25:41.9145061Z Author       : Microsoft
2020-02-19T04:25:41.9145061Z Author       : Microsoft
2020-02-19T04:25:41.9145110Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-19T04:25:41.9145250Z ==============================================================================
2020-02-19T04:25:42.2979073Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-19T04:25:42.3017871Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-02-19T04:25:42.3123538Z Cleaning up task key
2020-02-19T04:25:42.3124381Z Start cleaning up orphan processes.
2020-02-19T04:25:42.3396996Z Terminate orphan process: pid (6517) (python)
2020-02-19T04:25:42.3416954Z ##[section]Finishing: Finalize Job
