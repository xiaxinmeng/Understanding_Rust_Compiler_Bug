plain
2019-07-19T15:10:26.2267721Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T15:10:26.2465544Z ##[command]git config gc.auto 0
2019-07-19T15:10:26.2514559Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T15:10:26.2572076Z ##[command]git config --get-all http.proxy
2019-07-19T15:10:26.2702929Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62655/merge:refs/remotes/pull/62655/merge
---
2019-07-19T15:11:00.6313254Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T15:11:00.6313294Z 
2019-07-19T15:11:00.6313462Z   git checkout -b <new-branch-name>
2019-07-19T15:11:00.6313483Z 
2019-07-19T15:11:00.6313519Z HEAD is now at 54c665fbc Merge b1159d72ee78d1abed86eef8589fb61de27696e1 into 527dce7137f7a3c7bf47d9a503abf25f88ea22de
2019-07-19T15:11:00.6442404Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T15:11:00.6445159Z ==============================================================================
2019-07-19T15:11:00.6445222Z Task         : Bash
2019-07-19T15:11:00.6445287Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T15:19:47.4063604Z    Compiling rand_pcg v0.1.1
2019-07-19T15:19:47.8115367Z    Compiling rand v0.6.1
2019-07-19T15:19:49.2477925Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-07-19T15:19:50.4229458Z     Checking tempfile v3.0.5
2019-07-19T15:19:52.6329369Z error[E0615]: attempted to take value of method `relocations` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.6329881Z    |
2019-07-19T15:19:52.6329881Z    |
2019-07-19T15:19:52.6330153Z 28 |     let mut llvals = Vec::with_capacity(alloc.relocations.len() + 1);
2019-07-19T15:19:52.6330847Z    |                                               ^^^^^^^^^^^ help: use parentheses to call the method: `relocations(...)`
2019-07-19T15:19:52.6330893Z 
2019-07-19T15:19:52.6331190Z error[E0615]: attempted to take value of method `relocations` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.6331649Z    |
2019-07-19T15:19:52.6331649Z    |
2019-07-19T15:19:52.6331922Z 33 |     for &(offset, ((), alloc_id)) in alloc.relocations.iter() {
2019-07-19T15:19:52.6332245Z    |                                            ^^^^^^^^^^^ help: use parentheses to call the method: `relocations(...)`
2019-07-19T15:19:52.6332311Z 
2019-07-19T15:19:52.6876269Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.6877901Z    |
2019-07-19T15:19:52.6877901Z    |
2019-07-19T15:19:52.6878575Z 38 |             llvals.push(cx.const_bytes(&alloc.alloc.bytes[next_offset..offset]));
2019-07-19T15:19:52.6879594Z 
2019-07-19T15:19:52.6879594Z 
2019-07-19T15:19:52.6885908Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.6886997Z    |
2019-07-19T15:19:52.6886997Z    |
2019-07-19T15:19:52.6887519Z 42 |             &alloc.alloc.bytes[offset..(offset + pointer_size)],
2019-07-19T15:19:52.6888533Z 
2019-07-19T15:19:52.6888533Z 
2019-07-19T15:19:52.6906478Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.6907637Z    |
2019-07-19T15:19:52.6907637Z    |
2019-07-19T15:19:52.6908417Z 54 |     if alloc.alloc.bytes.len() >= next_offset {
2019-07-19T15:19:52.6908952Z 
2019-07-19T15:19:52.6908952Z 
2019-07-19T15:19:52.6915970Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.6917058Z    |
2019-07-19T15:19:52.6917058Z    |
2019-07-19T15:19:52.6917622Z 55 |         llvals.push(cx.const_bytes(&alloc.alloc.bytes[next_offset ..]));
2019-07-19T15:19:52.6918460Z 
2019-07-19T15:19:52.6918460Z 
2019-07-19T15:19:52.7190025Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.7191455Z     |
2019-07-19T15:19:52.7191455Z     |
2019-07-19T15:19:52.7191933Z 440 |                     let sect_name = if alloc.alloc.bytes.iter().all(|b| *b == 0) {
2019-07-19T15:19:52.7192792Z 
2019-07-19T15:19:52.7192792Z 
2019-07-19T15:19:52.7205003Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.7206165Z     |
2019-07-19T15:19:52.7206165Z     |
2019-07-19T15:19:52.7206684Z 461 |                         alloc.alloc.bytes.as_ptr() as *const _,
2019-07-19T15:19:52.7207343Z 
2019-07-19T15:19:52.7207343Z 
2019-07-19T15:19:52.7211241Z error[E0609]: no field `alloc` on type `&rustc::mir::interpret::Allocation`
2019-07-19T15:19:52.7212495Z     |
2019-07-19T15:19:52.7212495Z     |
2019-07-19T15:19:52.7212921Z 462 |                         alloc.alloc.bytes.len() as c_uint,
2019-07-19T15:19:52.7214102Z 
2019-07-19T15:19:54.0718804Z error: aborting due to 9 previous errors
2019-07-19T15:19:54.0718951Z 
2019-07-19T15:19:54.0719204Z Some errors have detailed explanations: E0609, E0615.
2019-07-19T15:19:54.0719204Z Some errors have detailed explanations: E0609, E0615.
2019-07-19T15:19:54.0719433Z For more information about an error, try `rustc --explain E0609`.
2019-07-19T15:19:54.0998693Z error: Could not compile `rustc_codegen_llvm`.
2019-07-19T15:19:54.0998790Z 
2019-07-19T15:19:54.0999025Z To learn more, run the command again with --verbose.
2019-07-19T15:19:54.1022570Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--message-format" "json"
2019-07-19T15:19:54.1037381Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-19T15:19:54.1037455Z Build completed unsuccessfully in 0:05:47
2019-07-19T15:19:54.1037455Z Build completed unsuccessfully in 0:05:47
2019-07-19T15:19:55.7290461Z ##[error]Bash exited with code '1'.
2019-07-19T15:19:55.7320916Z ##[section]Starting: Checkout
2019-07-19T15:19:55.7322928Z ==============================================================================
2019-07-19T15:19:55.7322975Z Task         : Get sources
2019-07-19T15:19:55.7323017Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
