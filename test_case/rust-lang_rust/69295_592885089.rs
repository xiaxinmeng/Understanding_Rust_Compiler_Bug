plain
2020-02-29T05:04:53.0306773Z ========================== Starting Command Output ===========================
2020-02-29T05:04:53.0324101Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4ce5820e-a951-4ba2-8989-666518870918.sh
2020-02-29T05:04:53.2905570Z 
2020-02-29T05:04:53.2967547Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T05:04:53.2996940Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69295/merge to s
2020-02-29T05:04:53.3005888Z Task         : Get sources
2020-02-29T05:04:53.3006363Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T05:04:53.3006782Z Version      : 1.0.0
2020-02-29T05:04:53.3007076Z Author       : Microsoft
---
2020-02-29T05:04:55.8177722Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T05:04:55.8332777Z ##[command]git config gc.auto 0
2020-02-29T05:04:55.8368514Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T05:04:55.8394237Z ##[command]git config --get-all http.proxy
2020-02-29T05:04:55.8473484Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69295/merge:refs/remotes/pull/69295/merge
---
2020-02-29T05:14:04.2295738Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-02-29T05:14:05.4064638Z error[E0061]: this function takes 2 parameters but 3 parameters were supplied
2020-02-29T05:14:05.4065348Z    --> src/librustc_mir/transform/generator.rs:623:22
2020-02-29T05:14:05.4065778Z     |
2020-02-29T05:14:05.4067182Z 623 |       requires_storage.visit_in_rpo_with(body, traversal::preorder(body), &mut visitor);
2020-02-29T05:14:05.4068595Z     | 
2020-02-29T05:14:05.4069063Z    ::: src/librustc_mir/dataflow/generic/mod.rs:88:5
2020-02-29T05:14:05.4069485Z     |
2020-02-29T05:14:05.4070036Z 88  | /     pub fn visit_in_rpo_with(
2020-02-29T05:14:05.4070036Z 88  | /     pub fn visit_in_rpo_with(
2020-02-29T05:14:05.4070821Z 89  | |         &self,
2020-02-29T05:14:05.4071526Z 90  | |         body: &'mir mir::Body<'tcx>,
2020-02-29T05:14:05.4072437Z 91  | |         vis: &mut impl ResultsVisitor<'mir, 'tcx, FlowState = BitSet<A::Idx>>,
2020-02-29T05:14:05.4073084Z ...   |
2020-02-29T05:14:05.4073718Z 94  | |         visit_results(body, blocks.map(|(bb, _)| bb), self, vis)
2020-02-29T05:14:05.4074979Z     | |_____- defined here
2020-02-29T05:14:05.4075216Z 
2020-02-29T05:14:06.1067773Z error: aborting due to previous error
2020-02-29T05:14:06.1068023Z 
2020-02-29T05:14:06.1068023Z 
2020-02-29T05:14:06.1071588Z For more information about this error, try `rustc --explain E0061`.
2020-02-29T05:14:06.1172943Z error: could not compile `rustc_mir`.
2020-02-29T05:14:06.1194876Z warning: build failed, waiting for other jobs to finish...
2020-02-29T05:14:08.8672422Z error: build failed
2020-02-29T05:14:08.8698485Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-29T05:14:08.8708704Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-29T05:14:08.8709003Z Build completed unsuccessfully in 0:05:15
2020-02-29T05:14:08.8752357Z == clock drift check ==
2020-02-29T05:14:08.8768013Z   local time: Sat Feb 29 05:14:08 UTC 2020
2020-02-29T05:14:08.8768013Z   local time: Sat Feb 29 05:14:08 UTC 2020
2020-02-29T05:14:09.0115689Z   network time: Sat, 29 Feb 2020 05:14:09 GMT
2020-02-29T05:14:09.0118672Z == end clock drift check ==
2020-02-29T05:14:09.7711914Z 
2020-02-29T05:14:09.7782794Z ##[error]Bash exited with code '1'.
2020-02-29T05:14:09.7794712Z ##[section]Finishing: Run build
2020-02-29T05:14:09.7830838Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69295/merge to s
2020-02-29T05:14:09.7838358Z Task         : Get sources
2020-02-29T05:14:09.7838800Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T05:14:09.7839059Z Version      : 1.0.0
2020-02-29T05:14:09.7839425Z Author       : Microsoft
2020-02-29T05:14:09.7839425Z Author       : Microsoft
2020-02-29T05:14:09.7839687Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-29T05:14:09.7839990Z ==============================================================================
2020-02-29T05:14:10.0885505Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-29T05:14:10.0924003Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69295/merge to s
2020-02-29T05:14:10.1008493Z Cleaning up task key
2020-02-29T05:14:10.1009823Z Start cleaning up orphan processes.
2020-02-29T05:14:10.1279578Z Terminate orphan process: pid (4892) (python)
2020-02-29T05:14:10.1321717Z ##[section]Finishing: Finalize Job
