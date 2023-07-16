plain
2019-07-26T09:59:06.0843246Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T09:59:06.1067524Z ##[command]git config gc.auto 0
2019-07-26T09:59:06.1132086Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T09:59:06.1193594Z ##[command]git config --get-all http.proxy
2019-07-26T09:59:06.1344068Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62969/merge:refs/remotes/pull/62969/merge
---
2019-07-26T09:59:41.7579108Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T09:59:41.7579150Z 
2019-07-26T09:59:41.7579422Z   git checkout -b <new-branch-name>
2019-07-26T09:59:41.7579482Z 
2019-07-26T09:59:41.7579543Z HEAD is now at eab2cc6a6 Merge f2526086a5fa8534db1e08980440e9ba853d8da5 into 4268e7ee22935f086b856ef0063a9e22b49aeddb
2019-07-26T09:59:41.7722029Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T09:59:41.7725354Z ==============================================================================
2019-07-26T09:59:41.7725428Z Task         : Bash
2019-07-26T09:59:41.7725485Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T10:07:08.3082056Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-26T10:07:09.7638270Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-26T10:07:11.0683486Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-26T10:07:23.9743587Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-26T10:07:27.6772355Z error[E0531]: cannot find tuple struct/variant `NoMirFor` in this scope
2019-07-26T10:07:27.6774637Z     |
2019-07-26T10:07:27.6774637Z     |
2019-07-26T10:07:27.6775447Z 444 |             NoMirFor(ref func) => write!(f, "no mir for `{}`", func),
2019-07-26T10:07:27.6776979Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:07:27.6777961Z     |
2019-07-26T10:07:27.6777961Z     |
2019-07-26T10:07:27.6778778Z 1   | use crate::mir::interpret::error::UndefinedBehaviourInfo::NoMirFor;
2019-07-26T10:07:27.6779812Z 
2019-07-26T10:07:27.6779812Z 
2019-07-26T10:07:27.6908177Z error[E0531]: cannot find tuple struct/variant `FunctionAbiMismatch` in this scope
2019-07-26T10:07:27.6911144Z     |
2019-07-26T10:07:27.6911144Z     |
2019-07-26T10:07:27.6911829Z 445 |             FunctionAbiMismatch(caller_abi, callee_abi) =>
2019-07-26T10:07:27.6913129Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:07:27.6913646Z     |
2019-07-26T10:07:27.6913646Z     |
2019-07-26T10:07:27.6914278Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionAbiMismatch;
2019-07-26T10:07:27.6915061Z 
2019-07-26T10:07:27.6915061Z 
2019-07-26T10:07:27.7039907Z error[E0531]: cannot find tuple struct/variant `FunctionArgMismatch` in this scope
2019-07-26T10:07:27.7041770Z     |
2019-07-26T10:07:27.7041770Z     |
2019-07-26T10:07:27.7042403Z 448 |             FunctionArgMismatch(caller_ty, callee_ty) =>
2019-07-26T10:07:27.7043634Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:07:27.7044558Z     |
2019-07-26T10:07:27.7044558Z     |
2019-07-26T10:07:27.7045328Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionArgMismatch;
2019-07-26T10:07:27.7046373Z 
2019-07-26T10:07:27.7046373Z 
2019-07-26T10:07:27.7184852Z error[E0531]: cannot find tuple struct/variant `FunctionRetMismatch` in this scope
2019-07-26T10:07:27.7257238Z     |
2019-07-26T10:07:27.7257238Z     |
2019-07-26T10:07:27.7257914Z 452 |             FunctionRetMismatch(caller_ty, callee_ty) =>
2019-07-26T10:07:27.7262643Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:07:27.7263156Z     |
2019-07-26T10:07:27.7263156Z     |
2019-07-26T10:07:27.7263614Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionRetMismatch;
2019-07-26T10:07:27.7264004Z 
2019-07-26T10:07:49.1538618Z error: aborting due to 4 previous errors
2019-07-26T10:07:49.1538810Z 
2019-07-26T10:07:49.3458690Z error: Could not compile `rustc`.
2019-07-26T10:07:49.3458690Z error: Could not compile `rustc`.
2019-07-26T10:07:49.3459701Z 
2019-07-26T10:07:49.3460436Z To learn more, run the command again with --verbose.
2019-07-26T10:07:49.3489448Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-26T10:07:49.3503931Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-26T10:07:49.3504374Z Build completed unsuccessfully in 0:05:00
2019-07-26T10:07:49.3504374Z Build completed unsuccessfully in 0:05:00
2019-07-26T10:07:50.1018812Z ##[error]Bash exited with code '1'.
2019-07-26T10:07:50.1055420Z ##[section]Starting: Checkout
2019-07-26T10:07:50.1058038Z ==============================================================================
2019-07-26T10:07:50.1058113Z Task         : Get sources
2019-07-26T10:07:50.1058191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
