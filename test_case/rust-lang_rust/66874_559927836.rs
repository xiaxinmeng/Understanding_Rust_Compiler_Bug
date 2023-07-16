plain
2019-11-30T08:24:12.0098047Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T08:24:12.0282195Z ##[command]git config gc.auto 0
2019-11-30T08:24:12.0349264Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T08:24:12.0405474Z ##[command]git config --get-all http.proxy
2019-11-30T08:24:12.0517831Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66874/merge:refs/remotes/pull/66874/merge
---
2019-11-30T08:35:39.2680502Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-30T08:38:08.3086556Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-11-30T08:38:46.6318121Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-11-30T08:39:20.3758946Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-30T08:39:21.4779266Z error[E0422]: cannot find struct, variant or union type `GlobalId` in this scope
2019-11-30T08:39:21.4779634Z    --> src/librustc_mir/interpret/terminator.rs:277:31
2019-11-30T08:39:21.4779829Z     |
2019-11-30T08:39:21.4780076Z 277 |                     let gid = GlobalId { instance, promoted: None };
2019-11-30T08:39:21.4780587Z     |
2019-11-30T08:39:21.4780840Z help: possible candidate is found in another module, you can import it into scope
2019-11-30T08:39:21.4781055Z     |
2019-11-30T08:39:21.4781313Z 1   | use rustc::mir::interpret::GlobalId;
2019-11-30T08:39:21.4781313Z 1   | use rustc::mir::interpret::GlobalId;
2019-11-30T08:39:21.4799589Z     |
2019-11-30T08:39:21.4799646Z 
2019-11-30T08:39:21.4907778Z error[E0412]: cannot find type `GlobalId` in this scope
2019-11-30T08:39:21.4908059Z    --> src/librustc_mir/interpret/terminator.rs:450:14
2019-11-30T08:39:21.4908300Z     |
2019-11-30T08:39:21.4908530Z 450 |         gid: GlobalId<'tcx>,
2019-11-30T08:39:21.4908984Z     |
2019-11-30T08:39:21.4909220Z help: possible candidate is found in another module, you can import it into scope
2019-11-30T08:39:21.4909402Z     |
2019-11-30T08:39:21.4909647Z 1   | use rustc::mir::interpret::GlobalId;
---
2019-11-30T08:40:48.8998394Z   local time: Sat Nov 30 08:40:48 UTC 2019
2019-11-30T08:40:49.1801441Z   network time: Sat, 30 Nov 2019 08:40:49 GMT
2019-11-30T08:40:49.1805141Z == end clock drift check ==
2019-11-30T08:40:52.0498019Z 
2019-11-30T08:40:52.0577646Z ##[error]Bash exited with code '1'.
2019-11-30T08:40:52.0605671Z ##[section]Starting: Checkout
2019-11-30T08:40:52.0608008Z ==============================================================================
2019-11-30T08:40:52.0608052Z Task         : Get sources
2019-11-30T08:40:52.0608090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
