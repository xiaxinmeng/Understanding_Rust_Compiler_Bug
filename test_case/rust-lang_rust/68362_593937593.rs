plain
2020-03-03T12:49:54.7899934Z ========================== Starting Command Output ===========================
2020-03-03T12:49:54.7902360Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d15c431f-2671-41b6-8ac3-7bba58d436ba.sh
2020-03-03T12:49:54.7902652Z 
2020-03-03T12:49:54.7906273Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T12:49:54.7926137Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T12:49:54.7930015Z Task         : Get sources
2020-03-03T12:49:54.7930326Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T12:49:54.7930623Z Version      : 1.0.0
2020-03-03T12:49:54.7930839Z Author       : Microsoft
---
2020-03-03T12:49:58.0361493Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T12:49:58.0386648Z ##[command]git config gc.auto 0
2020-03-03T12:49:58.0389234Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T12:49:58.0391421Z ##[command]git config --get-all http.proxy
2020-03-03T12:49:58.0396690Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-03-03T12:58:51.0852877Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2020-03-03T12:58:51.1734044Z error[E0515]: cannot return value referencing temporary value
2020-03-03T12:58:51.1734855Z    --> src/librustc_infer/infer/region_constraints/mod.rs:151:38
2020-03-03T12:58:51.1735397Z     |
2020-03-03T12:58:51.1736264Z 151 |             Self::VarSubVar(a, b) => OutlivesPredicate(&RegionKind::ReVar(*a), &RegionKind::ReVar(*b)),
2020-03-03T12:58:51.1739019Z     |                                      |                                          |
2020-03-03T12:58:51.1740109Z     |                                      |                                          temporary value created here
2020-03-03T12:58:51.1741158Z     |                                      returns a value referencing data owned by the current function
2020-03-03T12:58:51.1741552Z 
2020-03-03T12:58:51.1741552Z 
2020-03-03T12:58:51.1742094Z error[E0515]: cannot return value referencing temporary value
2020-03-03T12:58:51.1742809Z    --> src/librustc_infer/infer/region_constraints/mod.rs:151:38
2020-03-03T12:58:51.1743344Z     |
2020-03-03T12:58:51.1744184Z 151 |             Self::VarSubVar(a, b) => OutlivesPredicate(&RegionKind::ReVar(*a), &RegionKind::ReVar(*b)),
2020-03-03T12:58:51.1746349Z     |                                      |                  |
2020-03-03T12:58:51.1747319Z     |                                      |                  temporary value created here
2020-03-03T12:58:51.1748326Z     |                                      returns a value referencing data owned by the current function
2020-03-03T12:58:51.1748724Z 
2020-03-03T12:58:51.1748724Z 
2020-03-03T12:58:51.1769516Z error[E0515]: cannot return value referencing temporary value
2020-03-03T12:58:51.1770395Z    --> src/librustc_infer/infer/region_constraints/mod.rs:152:38
2020-03-03T12:58:51.1770956Z     |
2020-03-03T12:58:51.1771745Z 152 |             Self::RegSubVar(a, b) => OutlivesPredicate(a, &RegionKind::ReVar(*b)),
2020-03-03T12:58:51.1773913Z     |                                      |                     |
2020-03-03T12:58:51.1774923Z     |                                      |                     temporary value created here
2020-03-03T12:58:51.1776030Z     |                                      returns a value referencing data owned by the current function
2020-03-03T12:58:51.1776429Z 
2020-03-03T12:58:51.1776429Z 
2020-03-03T12:58:51.1778081Z error[E0515]: cannot return value referencing temporary value
2020-03-03T12:58:51.1778976Z    --> src/librustc_infer/infer/region_constraints/mod.rs:153:38
2020-03-03T12:58:51.1779689Z     |
2020-03-03T12:58:51.1813676Z 153 |             Self::VarSubReg(a, b) => OutlivesPredicate(&RegionKind::ReVar(*a), b),
2020-03-03T12:58:51.1816398Z     |                                      |                  |
2020-03-03T12:58:51.1817556Z     |                                      |                  temporary value created here
2020-03-03T12:58:51.1818678Z     |                                      returns a value referencing data owned by the current function
2020-03-03T12:58:51.1819221Z 
2020-03-03T12:58:51.1819221Z 
2020-03-03T12:58:52.5179662Z error: aborting due to 4 previous errors
2020-03-03T12:58:52.5180309Z 
2020-03-03T12:58:52.5181061Z For more information about this error, try `rustc --explain E0515`.
2020-03-03T12:58:52.5257081Z error: could not compile `rustc_infer`.
2020-03-03T12:58:52.5275420Z warning: build failed, waiting for other jobs to finish...
2020-03-03T12:58:55.3606068Z error: build failed
2020-03-03T12:58:55.3633356Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-03T12:58:55.3643738Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-03T12:58:55.3644472Z Build completed unsuccessfully in 0:04:52
2020-03-03T12:58:55.3694342Z == clock drift check ==
2020-03-03T12:58:55.3708782Z   local time: Tue Mar  3 12:58:55 UTC 2020
2020-03-03T12:58:55.3708782Z   local time: Tue Mar  3 12:58:55 UTC 2020
2020-03-03T12:58:55.6544452Z   network time: Tue, 03 Mar 2020 12:58:55 GMT
2020-03-03T12:58:55.6547177Z == end clock drift check ==
2020-03-03T12:58:56.3397715Z 
2020-03-03T12:58:56.3476053Z ##[error]Bash exited with code '1'.
2020-03-03T12:58:56.3490458Z ##[section]Finishing: Run build
2020-03-03T12:58:56.3535406Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T12:58:56.3540294Z Task         : Get sources
2020-03-03T12:58:56.3540636Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T12:58:56.3540962Z Version      : 1.0.0
2020-03-03T12:58:56.3541184Z Author       : Microsoft
2020-03-03T12:58:56.3541184Z Author       : Microsoft
2020-03-03T12:58:56.3541537Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T12:58:56.3541950Z ==============================================================================
2020-03-03T12:58:56.6354959Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T12:58:56.6395833Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T12:58:56.6478719Z Cleaning up task key
2020-03-03T12:58:56.6479874Z Start cleaning up orphan processes.
2020-03-03T12:58:56.6727067Z Terminate orphan process: pid (4166) (python)
2020-03-03T12:58:56.6761149Z ##[section]Finishing: Finalize Job
