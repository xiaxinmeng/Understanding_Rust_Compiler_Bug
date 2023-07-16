plain
2020-03-03T18:13:24.4092011Z ========================== Starting Command Output ===========================
2020-03-03T18:13:24.4097691Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/547a81e6-5307-4201-85ee-f30e5020a9dd.sh
2020-03-03T18:13:24.4098006Z 
2020-03-03T18:13:24.4104884Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T18:13:24.4130124Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-03-03T18:13:24.4137337Z Task         : Get sources
2020-03-03T18:13:24.4137707Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T18:13:24.4138049Z Version      : 1.0.0
2020-03-03T18:13:24.4138278Z Author       : Microsoft
---
2020-03-03T18:13:25.4067000Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T18:13:25.4072712Z ##[command]git config gc.auto 0
2020-03-03T18:13:25.4080396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T18:13:25.4085662Z ##[command]git config --get-all http.proxy
2020-03-03T18:13:25.4092603Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69326/merge:refs/remotes/pull/69326/merge
---
2020-03-03T18:22:09.7400469Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-03-03T18:22:10.3366843Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-03-03T18:22:10.7176810Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-03T18:22:12.0259539Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-03-03T18:22:16.0161840Z error[E0277]: the trait bound `&interpret::memory::Memory<'mir, 'tcx, M>: rustc_target::abi::HasDataLayout` is not satisfied
2020-03-03T18:22:16.0162799Z    --> src/librustc_mir/interpret/memory.rs:812:28
2020-03-03T18:22:16.0163358Z     |
2020-03-03T18:22:16.0164111Z 812 |                 allocation.read_scalar(&self, ptr, self.data_layout().pointer_size)?.to_u16()?;
2020-03-03T18:22:16.0165437Z     |                            ^^^^^^^^^^^ the trait `rustc_target::abi::HasDataLayout` is not implemented for `&interpret::memory::Memory<'mir, 'tcx, M>`
2020-03-03T18:22:16.0167015Z     = help: the following implementations were found:
2020-03-03T18:22:16.0167015Z     = help: the following implementations were found:
2020-03-03T18:22:16.0167846Z               <interpret::memory::Memory<'mir, 'tcx, M> as rustc_target::abi::HasDataLayout>
2020-03-03T18:22:16.0168164Z 
2020-03-03T18:22:16.0330161Z error[E0277]: the trait bound `&interpret::memory::Memory<'mir, 'tcx, M>: rustc_target::abi::HasDataLayout` is not satisfied
2020-03-03T18:22:16.0331151Z    --> src/librustc_mir/interpret/memory.rs:815:51
2020-03-03T18:22:16.0331867Z     |
2020-03-03T18:22:16.0332644Z 815 |                 ptr = ptr.offset(Size { raw: 2 }, &self);
2020-03-03T18:22:16.0333551Z     |                                                   -^^^^
2020-03-03T18:22:16.0334375Z     |                                                   |
2020-03-03T18:22:16.0335571Z     |                                                   the trait `rustc_target::abi::HasDataLayout` is not implemented for `&interpret::memory::Memory<'mir, 'tcx, M>`
2020-03-03T18:22:16.0336814Z     |                                                   help: consider removing 1 leading `&`-references
2020-03-03T18:22:16.0338199Z     = help: the following implementations were found:
2020-03-03T18:22:16.0338199Z     = help: the following implementations were found:
2020-03-03T18:22:16.0338999Z               <interpret::memory::Memory<'mir, 'tcx, M> as rustc_target::abi::HasDataLayout>
2020-03-03T18:22:16.1588316Z error[E0308]: mismatched types
2020-03-03T18:22:16.1589132Z    --> src/librustc_mir/interpret/memory.rs:815:23
2020-03-03T18:22:16.1589964Z     |
2020-03-03T18:22:16.1589964Z     |
2020-03-03T18:22:16.1590709Z 815 |                 ptr = ptr.offset(Size { raw: 2 }, &self);
2020-03-03T18:22:16.1592873Z     |
2020-03-03T18:22:16.1593672Z     = note: expected struct `rustc::mir::interpret::Pointer<_>`
2020-03-03T18:22:16.1594753Z                  found enum `std::result::Result<rustc::mir::interpret::Pointer<_>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2020-03-03T18:22:16.1595256Z 
---
2020-03-03T18:22:18.6523286Z For more information about an error, try `rustc --explain E0277`.
2020-03-03T18:22:18.6711651Z error: could not compile `rustc_mir`.
2020-03-03T18:22:18.6712136Z 
2020-03-03T18:22:18.6712654Z To learn more, run the command again with --verbose.
2020-03-03T18:22:18.6748277Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-03T18:22:18.6757940Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-03T18:22:18.6758295Z Build completed unsuccessfully in 0:05:47
2020-03-03T18:22:18.6807321Z == clock drift check ==
2020-03-03T18:22:18.6816029Z   local time: Tue Mar  3 18:22:18 UTC 2020
2020-03-03T18:22:18.6816029Z   local time: Tue Mar  3 18:22:18 UTC 2020
2020-03-03T18:22:18.9653126Z   network time: Tue, 03 Mar 2020 18:22:18 GMT
2020-03-03T18:22:18.9658476Z == end clock drift check ==
2020-03-03T18:22:19.7769178Z 
2020-03-03T18:22:19.7863255Z ##[error]Bash exited with code '1'.
2020-03-03T18:22:19.7880653Z ##[section]Finishing: Run build
2020-03-03T18:22:19.7936996Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-03-03T18:22:19.7943512Z Task         : Get sources
2020-03-03T18:22:19.7943937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T18:22:19.7944320Z Version      : 1.0.0
2020-03-03T18:22:19.7944571Z Author       : Microsoft
2020-03-03T18:22:19.7944571Z Author       : Microsoft
2020-03-03T18:22:19.7944993Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T18:22:19.7945453Z ==============================================================================
2020-03-03T18:22:20.1219394Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T18:22:20.1265716Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-03-03T18:22:20.1354950Z Cleaning up task key
2020-03-03T18:22:20.1356317Z Start cleaning up orphan processes.
2020-03-03T18:22:20.1532815Z Terminate orphan process: pid (3599) (python)
2020-03-03T18:22:20.1682461Z ##[section]Finishing: Finalize Job
