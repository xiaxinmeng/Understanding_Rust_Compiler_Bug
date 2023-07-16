plain
2020-02-26T10:25:00.7169308Z ========================== Starting Command Output ===========================
2020-02-26T10:25:00.7171724Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bb697d82-96b9-4669-8b09-724483e23eca.sh
2020-02-26T10:25:00.7171953Z 
2020-02-26T10:25:00.7174752Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T10:25:00.7192279Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:25:00.7195495Z Task         : Get sources
2020-02-26T10:25:00.7195791Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T10:25:00.7196033Z Version      : 1.0.0
2020-02-26T10:25:00.7196196Z Author       : Microsoft
---
2020-02-26T10:25:02.9521839Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T10:25:02.9527059Z ##[command]git config gc.auto 0
2020-02-26T10:25:02.9530382Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T10:25:02.9533750Z ##[command]git config --get-all http.proxy
2020-02-26T10:25:02.9540050Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-02-26T10:32:52.8540038Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-02-26T10:32:55.0734341Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2020-02-26T10:32:55.0735322Z    --> src/librustc_target/abi/call/mod.rs:581:22
2020-02-26T10:32:55.0736024Z     |
2020-02-26T10:32:55.0736839Z 581 |             "avr" => avr::compute_abi_info(cx, self),
2020-02-26T10:32:55.0738680Z     | 
2020-02-26T10:32:55.0739227Z    ::: src/librustc_target/abi/call/avr.rs:21:1
2020-02-26T10:32:55.0739700Z     |
2020-02-26T10:32:55.0739700Z     |
2020-02-26T10:32:55.0740335Z 21  | pub fn compute_abi_info<Ty>(fty: &mut FnAbi<'_, Ty>) {
2020-02-26T10:32:55.0744031Z 
2020-02-26T10:32:55.9064134Z error: aborting due to previous error
2020-02-26T10:32:55.9064523Z 
2020-02-26T10:32:55.9072331Z For more information about this error, try `rustc --explain E0061`.
2020-02-26T10:32:55.9072331Z For more information about this error, try `rustc --explain E0061`.
2020-02-26T10:32:55.9112678Z error: could not compile `rustc_target`.
2020-02-26T10:32:55.9113566Z warning: build failed, waiting for other jobs to finish...
2020-02-26T10:32:58.6316796Z error: build failed
2020-02-26T10:32:58.6338284Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-26T10:32:58.6351959Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-26T10:32:58.6352631Z Build completed unsuccessfully in 0:04:34
2020-02-26T10:32:58.6400516Z == clock drift check ==
2020-02-26T10:32:58.6410723Z   local time: Wed Feb 26 10:32:58 UTC 2020
2020-02-26T10:32:58.6410723Z   local time: Wed Feb 26 10:32:58 UTC 2020
2020-02-26T10:32:58.8235687Z   network time: Wed, 26 Feb 2020 10:32:58 GMT
2020-02-26T10:32:58.8242989Z == end clock drift check ==
2020-02-26T10:32:59.5771819Z 
2020-02-26T10:32:59.5837674Z ##[error]Bash exited with code '1'.
2020-02-26T10:32:59.5850354Z ##[section]Finishing: Run build
2020-02-26T10:32:59.5892733Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:32:59.5897183Z Task         : Get sources
2020-02-26T10:32:59.5897485Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T10:32:59.5897760Z Version      : 1.0.0
2020-02-26T10:32:59.5897967Z Author       : Microsoft
2020-02-26T10:32:59.5897967Z Author       : Microsoft
2020-02-26T10:32:59.5898274Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T10:32:59.5898627Z ==============================================================================
2020-02-26T10:32:59.9012316Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-26T10:32:59.9055742Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:32:59.9143872Z Cleaning up task key
2020-02-26T10:32:59.9145008Z Start cleaning up orphan processes.
2020-02-26T10:32:59.9416052Z Terminate orphan process: pid (3805) (python)
2020-02-26T10:32:59.9459783Z ##[section]Finishing: Finalize Job
