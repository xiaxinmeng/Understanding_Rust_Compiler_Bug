plain
2020-01-12T10:50:59.9893631Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-12T10:50:59.9903654Z ##[command]git config gc.auto 0
2020-01-12T10:50:59.9906703Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-12T10:50:59.9908490Z ##[command]git config --get-all http.proxy
2020-01-12T10:50:59.9910800Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67940/merge:refs/remotes/pull/67940/merge
---
2020-01-12T12:36:54.6570712Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/feature_gate/index.html"
2020-01-12T12:36:54.6570890Z 
2020-01-12T12:36:54.6571732Z     ┌── stabilization_guide.md:94:1 ───
2020-01-12T12:36:54.6572072Z     │
2020-01-12T12:36:54.6572642Z  94 │ [`src/libsyntax/feature_gate.rs`]. Search for the `declare_features!`
2020-01-12T12:36:54.6573623Z     │
2020-01-12T12:36:54.6573798Z 
2020-01-12T12:36:54.6573798Z 
2020-01-12T12:36:54.6574460Z error: Unable to retrieve "http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf": https://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf: error trying to connect: error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:ssl/statem/statem_clnt.c:1915: (unable to get local issuer certificate)
2020-01-12T12:36:54.6575182Z     ┌── appendix/bibliography.md:33:3 ───
2020-01-12T12:36:54.6577970Z     │
2020-01-12T12:36:54.6577970Z     │
2020-01-12T12:36:54.6578597Z  33 │ * [Non-blocking steal-half work queues](http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf)
2020-01-12T12:36:54.6579348Z     │   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ https://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf: error trying to connect: error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:ssl/statem/statem_clnt.c:1915: (unable to get local issuer certificate)
2020-01-12T12:36:54.6579902Z 
2020-01-12T12:36:54.6580045Z Error: One or more incorrect links
2020-01-12T12:36:54.6580230Z 
2020-01-12T12:36:54.6580374Z 
---
2020-01-12T13:02:30.0562483Z 
2020-01-12T13:02:30.0562894Z If you do intend to update 'rustc-guide', please check the error messages above and
2020-01-12T13:02:30.0563091Z commit another update.
2020-01-12T13:02:30.0563219Z 
2020-01-12T13:02:30.0563749Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2020-01-12T13:02:30.0564185Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2020-01-12T13:02:30.0564472Z proper steps.
2020-01-12T13:02:30.0569183Z Build completed unsuccessfully in 0:00:01
2020-01-12T13:02:30.0622212Z == clock drift check ==
2020-01-12T13:02:30.0640878Z   local time: Sun Jan 12 13:02:30 UTC 2020
2020-01-12T13:02:30.2080004Z   network time: Sun, 12 Jan 2020 13:02:30 GMT
2020-01-12T13:02:30.2080004Z   network time: Sun, 12 Jan 2020 13:02:30 GMT
2020-01-12T13:02:30.2080888Z == end clock drift check ==
2020-01-12T13:02:31.4401130Z 
2020-01-12T13:02:31.4524227Z ##[error]Bash exited with code '1'.
2020-01-12T13:02:31.4564493Z ##[section]Starting: Checkout
2020-01-12T13:02:31.4566866Z ==============================================================================
2020-01-12T13:02:31.4566943Z Task         : Get sources
2020-01-12T13:02:31.4566993Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
