plain
2019-09-06T22:19:13.5886905Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T22:19:13.6075933Z ##[command]git config gc.auto 0
2019-09-06T22:19:13.6160867Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T22:19:13.6204520Z ##[command]git config --get-all http.proxy
2019-09-06T22:19:13.6342651Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64226/merge:refs/remotes/pull/64226/merge
---
2019-09-06T22:26:13.2481707Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-09-06T22:26:21.0946231Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-06T22:26:22.3388742Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-06T22:26:23.4816153Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-06T22:26:28.7383109Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:28.7384265Z      |
2019-09-06T22:26:28.7384265Z      |
2019-09-06T22:26:28.7384751Z 1375 |                     self.recover_stmt();
2019-09-06T22:26:28.7385703Z 
2019-09-06T22:26:28.7385703Z 
2019-09-06T22:26:28.7531954Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:28.7533562Z      |
2019-09-06T22:26:28.7533562Z      |
2019-09-06T22:26:28.7534039Z 1582 |                         self.recover_stmt();
2019-09-06T22:26:28.7534747Z 
2019-09-06T22:26:28.7534747Z 
2019-09-06T22:26:28.7543550Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:28.7544566Z      |
2019-09-06T22:26:28.7544566Z      |
2019-09-06T22:26:28.7545231Z 1598 |                     self.recover_stmt();
2019-09-06T22:26:28.7545976Z 
2019-09-06T22:26:28.7545976Z 
2019-09-06T22:26:28.7977278Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:28.7978781Z    --> src/libsyntax/parse/parser/pat.rs:691:18
2019-09-06T22:26:28.7980024Z     |
2019-09-06T22:26:28.7980702Z 691 |             self.recover_stmt();
2019-09-06T22:26:28.7981598Z 
2019-09-06T22:26:28.7981598Z 
2019-09-06T22:26:28.9521264Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:28.9523271Z      |
2019-09-06T22:26:28.9523271Z      |
2019-09-06T22:26:28.9523761Z 1532 |             self.recover_stmt();
2019-09-06T22:26:28.9524479Z 
2019-09-06T22:26:28.9524479Z 
2019-09-06T22:26:28.9689683Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:28.9691122Z      |
2019-09-06T22:26:28.9691122Z      |
2019-09-06T22:26:28.9691551Z 1695 |                     self.recover_stmt();
2019-09-06T22:26:28.9692086Z 
2019-09-06T22:26:28.9692086Z 
2019-09-06T22:26:29.0764339Z error[E0624]: method `recover_stmt` is private
2019-09-06T22:26:29.0765311Z     |
2019-09-06T22:26:29.0765311Z     |
2019-09-06T22:26:29.0765686Z 443 |                         self.recover_stmt();
2019-09-06T22:26:29.0766217Z 
2019-09-06T22:26:30.3130970Z error: aborting due to 7 previous errors
2019-09-06T22:26:30.3131546Z 
2019-09-06T22:26:30.3132052Z For more information about this error, try `rustc --explain E0624`.
---
2019-09-06T22:26:30.3742391Z == clock drift check ==
2019-09-06T22:26:30.3756470Z   local time: Fri Sep  6 22:26:30 UTC 2019
2019-09-06T22:26:30.4596313Z   network time: Fri, 06 Sep 2019 22:26:30 GMT
2019-09-06T22:26:30.4599931Z == end clock drift check ==
2019-09-06T22:26:31.8644059Z ##[error]Bash exited with code '1'.
2019-09-06T22:26:31.8676370Z ##[section]Starting: Checkout
2019-09-06T22:26:31.8677867Z ==============================================================================
2019-09-06T22:26:31.8677930Z Task         : Get sources
2019-09-06T22:26:31.8677968Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
