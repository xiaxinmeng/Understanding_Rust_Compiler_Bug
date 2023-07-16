plain
2020-03-03T18:23:01.8409791Z ========================== Starting Command Output ===========================
2020-03-03T18:23:01.8412290Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b2c9d166-9ac5-4260-afbd-fa6a6153325a.sh
2020-03-03T18:23:01.8412564Z 
2020-03-03T18:23:01.8431414Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T18:23:01.8450052Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-03-03T18:23:01.8452855Z Task         : Get sources
2020-03-03T18:23:01.8453105Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T18:23:01.8453363Z Version      : 1.0.0
2020-03-03T18:23:01.8453529Z Author       : Microsoft
---
2020-03-03T18:23:02.8466006Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T18:23:02.8471427Z ##[command]git config gc.auto 0
2020-03-03T18:23:02.8476358Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T18:23:02.8481027Z ##[command]git config --get-all http.proxy
2020-03-03T18:23:02.8488659Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69326/merge:refs/remotes/pull/69326/merge
---
2020-03-03T18:31:53.8234732Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-03T18:31:55.5269484Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-03-03T18:31:55.9651383Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-03-03T18:31:56.2312345Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-03-03T18:31:57.5697966Z error[E0277]: the trait bound `&interpret::memory::Memory<'mir, 'tcx, M>: rustc_target::abi::HasDataLayout` is not satisfied
2020-03-03T18:31:57.5698654Z    --> src/librustc_mir/interpret/memory.rs:812:28
2020-03-03T18:31:57.5699063Z     |
2020-03-03T18:31:57.5735570Z 812 |                 allocation.read_scalar(&self, ptr, self.data_layout().pointer_size)?.to_u16()?;
2020-03-03T18:31:57.5736859Z     |                            ^^^^^^^^^^^ the trait `rustc_target::abi::HasDataLayout` is not implemented for `&interpret::memory::Memory<'mir, 'tcx, M>`
2020-03-03T18:31:57.5739547Z     = help: the following implementations were found:
2020-03-03T18:31:57.5739547Z     = help: the following implementations were found:
2020-03-03T18:31:57.5740481Z               <interpret::memory::Memory<'mir, 'tcx, M> as rustc_target::abi::HasDataLayout>
2020-03-03T18:31:57.5740747Z 
2020-03-03T18:31:57.5787944Z error[E0277]: the trait bound `&interpret::memory::Memory<'mir, 'tcx, M>: rustc_target::abi::HasDataLayout` is not satisfied
2020-03-03T18:31:57.5788591Z    --> src/librustc_mir/interpret/memory.rs:815:51
2020-03-03T18:31:57.5788985Z     |
2020-03-03T18:31:57.5789482Z 815 |                 ptr = ptr.offset(Size { raw: 2 }, &self);
2020-03-03T18:31:57.5790119Z     |                                                   -^^^^
2020-03-03T18:31:57.5790712Z     |                                                   |
2020-03-03T18:31:57.5791530Z     |                                                   the trait `rustc_target::abi::HasDataLayout` is not implemented for `&interpret::memory::Memory<'mir, 'tcx, M>`
2020-03-03T18:31:57.5792406Z     |                                                   help: consider removing 1 leading `&`-references
2020-03-03T18:31:57.5793370Z     = help: the following implementations were found:
2020-03-03T18:31:57.5793370Z     = help: the following implementations were found:
2020-03-03T18:31:57.5793922Z               <interpret::memory::Memory<'mir, 'tcx, M> as rustc_target::abi::HasDataLayout>
2020-03-03T18:31:57.6543622Z error[E0308]: mismatched types
2020-03-03T18:31:57.6544148Z    --> src/librustc_mir/interpret/memory.rs:815:23
2020-03-03T18:31:57.6544542Z     |
2020-03-03T18:31:57.6544542Z     |
2020-03-03T18:31:57.6545057Z 815 |                 ptr = ptr.offset(Size { raw: 2 }, &self);
2020-03-03T18:31:57.6546554Z     |
2020-03-03T18:31:57.6547046Z     = note: expected struct `rustc::mir::interpret::Pointer<_>`
2020-03-03T18:31:57.6547781Z                  found enum `std::result::Result<rustc::mir::interpret::Pointer<_>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2020-03-03T18:31:57.6548138Z 
---
2020-03-03T18:31:59.4731338Z For more information about an error, try `rustc --explain E0277`.
2020-03-03T18:31:59.4788416Z error: could not compile `rustc_mir`.
2020-03-03T18:31:59.4789050Z warning: build failed, waiting for other jobs to finish...
2020-03-03T18:32:00.4803466Z error: build failed
2020-03-03T18:32:00.4830208Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-03T18:32:00.4843270Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-03T18:32:00.4843703Z Build completed unsuccessfully in 0:04:29
2020-03-03T18:32:00.4886348Z == clock drift check ==
2020-03-03T18:32:00.4906014Z   local time: Tue Mar  3 18:32:00 UTC 2020
2020-03-03T18:32:00.4906014Z   local time: Tue Mar  3 18:32:00 UTC 2020
2020-03-03T18:32:00.7770841Z   network time: Tue, 03 Mar 2020 18:32:00 GMT
2020-03-03T18:32:00.7774599Z == end clock drift check ==
2020-03-03T18:32:01.5299442Z 
2020-03-03T18:32:01.5360708Z ##[error]Bash exited with code '1'.
2020-03-03T18:32:01.5373935Z ##[section]Finishing: Run build
2020-03-03T18:32:01.5416362Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-03-03T18:32:01.5421499Z Task         : Get sources
2020-03-03T18:32:01.5422973Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T18:32:01.5423313Z Version      : 1.0.0
2020-03-03T18:32:01.5423583Z Author       : Microsoft
2020-03-03T18:32:01.5423583Z Author       : Microsoft
2020-03-03T18:32:01.5423924Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T18:32:01.5424326Z ==============================================================================
2020-03-03T18:32:01.8032588Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T18:32:01.8082268Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-03-03T18:32:01.8167296Z Cleaning up task key
2020-03-03T18:32:01.8168281Z Start cleaning up orphan processes.
2020-03-03T18:32:01.8363042Z Terminate orphan process: pid (4059) (python)
2020-03-03T18:32:01.8389799Z ##[section]Finishing: Finalize Job
