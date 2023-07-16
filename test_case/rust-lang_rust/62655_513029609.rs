plain
2019-07-18T23:11:31.9850321Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T23:11:32.0028838Z ##[command]git config gc.auto 0
2019-07-18T23:11:32.0116765Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T23:11:32.0168985Z ##[command]git config --get-all http.proxy
2019-07-18T23:11:32.0295810Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62655/merge:refs/remotes/pull/62655/merge
---
2019-07-18T23:12:06.4690489Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T23:12:06.4690534Z 
2019-07-18T23:12:06.4690718Z   git checkout -b <new-branch-name>
2019-07-18T23:12:06.4690744Z 
2019-07-18T23:12:06.4690786Z HEAD is now at d036a3f38 Merge 3a94e1a3faf0aea0cb2cd79ace82e60b5bb5805b into 311376d30dc1cfa622142a9f50317b1e0cb4608a
2019-07-18T23:12:06.4828962Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T23:12:06.4831765Z ==============================================================================
2019-07-18T23:12:06.4831812Z Task         : Bash
2019-07-18T23:12:06.4831850Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T23:20:56.4060964Z    Compiling rand_chacha v0.1.0
2019-07-18T23:20:56.7306233Z    Compiling rand v0.6.1
2019-07-18T23:20:56.8169733Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-07-18T23:20:59.2046546Z     Checking tempfile v3.0.5
2019-07-18T23:21:00.7172997Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7177899Z    |
2019-07-18T23:21:00.7177899Z    |
2019-07-18T23:21:00.7178746Z 38 |             llvals.push(cx.const_bytes(&alloc.alloc.bytes[next_offset..offset]));
2019-07-18T23:21:00.7179537Z 
2019-07-18T23:21:00.7179537Z 
2019-07-18T23:21:00.7180210Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7181400Z    |
2019-07-18T23:21:00.7181400Z    |
2019-07-18T23:21:00.7181859Z 42 |             &alloc.alloc.bytes[offset..(offset + pointer_size)],
2019-07-18T23:21:00.7182506Z 
2019-07-18T23:21:00.7182506Z 
2019-07-18T23:21:00.7194504Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7196600Z    |
2019-07-18T23:21:00.7196600Z    |
2019-07-18T23:21:00.7197155Z 54 |     if alloc.alloc.bytes.len() >= next_offset {
2019-07-18T23:21:00.7197996Z 
2019-07-18T23:21:00.7197996Z 
2019-07-18T23:21:00.7204785Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7206685Z    |
2019-07-18T23:21:00.7206685Z    |
2019-07-18T23:21:00.7207276Z 55 |         llvals.push(cx.const_bytes(&alloc.alloc.bytes[next_offset ..]));
2019-07-18T23:21:00.7208071Z 
2019-07-18T23:21:00.7208071Z 
2019-07-18T23:21:00.7483351Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7484743Z     |
2019-07-18T23:21:00.7484743Z     |
2019-07-18T23:21:00.7485752Z 440 |                     let sect_name = if alloc.alloc.bytes.iter().all(|b| *b == 0) {
2019-07-18T23:21:00.7487142Z 
2019-07-18T23:21:00.7487142Z 
2019-07-18T23:21:00.7500257Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7501386Z     |
2019-07-18T23:21:00.7501386Z     |
2019-07-18T23:21:00.7502197Z 461 |                         alloc.alloc.bytes.as_ptr() as *const _,
2019-07-18T23:21:00.7506032Z 
2019-07-18T23:21:00.7506032Z 
2019-07-18T23:21:00.7506742Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-18T23:21:00.7508018Z     |
2019-07-18T23:21:00.7508018Z     |
2019-07-18T23:21:00.7508515Z 462 |                         alloc.alloc.bytes.len() as c_uint,
2019-07-18T23:21:00.7509590Z 
2019-07-18T23:21:02.0518642Z error: aborting due to 7 previous errors
2019-07-18T23:21:02.0518830Z 
2019-07-18T23:21:02.0519432Z For more information about this error, try `rustc --explain E0609`.
2019-07-18T23:21:02.0519432Z For more information about this error, try `rustc --explain E0609`.
2019-07-18T23:21:02.0802094Z error: Could not compile `rustc_codegen_llvm`.
2019-07-18T23:21:02.0802197Z 
2019-07-18T23:21:02.0802433Z To learn more, run the command again with --verbose.
2019-07-18T23:21:02.0822827Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--message-format" "json"
2019-07-18T23:21:02.0837840Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-18T23:21:02.0839177Z Build completed unsuccessfully in 0:05:46
2019-07-18T23:21:02.0839177Z Build completed unsuccessfully in 0:05:46
2019-07-18T23:21:03.8561241Z ##[error]Bash exited with code '1'.
2019-07-18T23:21:03.8593942Z ##[section]Starting: Checkout
2019-07-18T23:21:03.8595962Z ==============================================================================
2019-07-18T23:21:03.8596020Z Task         : Get sources
2019-07-18T23:21:03.8596069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
