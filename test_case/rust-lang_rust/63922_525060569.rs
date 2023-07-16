plain
2019-08-26T19:49:50.4536730Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T19:49:50.4791461Z ##[command]git config gc.auto 0
2019-08-26T19:49:50.4863024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T19:49:50.4925290Z ##[command]git config --get-all http.proxy
2019-08-26T19:49:50.5095205Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63922/merge:refs/remotes/pull/63922/merge
---
2019-08-26T19:50:25.6617647Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T19:50:25.6617701Z 
2019-08-26T19:50:25.6617961Z   git checkout -b <new-branch-name>
2019-08-26T19:50:25.6618013Z 
2019-08-26T19:50:25.6618067Z HEAD is now at c5c6bd7d2 Merge c706ed740e5745d6e0db5a823b027131ff20e287 into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T19:50:25.6806078Z ##[section]Starting: Decide whether to run this job
2019-08-26T19:50:25.6809937Z ==============================================================================
2019-08-26T19:50:25.6810022Z Task         : Bash
2019-08-26T19:50:25.6810073Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T22:12:20.4890757Z    Compiling rustc-ap-arena v546.0.0
2019-08-26T22:12:21.3203793Z    Compiling rustc-ap-syntax_pos v546.0.0
2019-08-26T22:12:37.1575367Z    Compiling rustc-ap-rustc_errors v546.0.0
2019-08-26T22:21:28.1148955Z    Compiling racer v2.1.25
2019-08-26T22:23:02.9077379Z warning: use of deprecated item 'build::rustc::rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-08-26T22:23:02.9079441Z    |
2019-08-26T22:23:02.9079981Z 18 | extern crate rustc_plugin;
2019-08-26T22:23:02.9080484Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-26T22:23:02.9080929Z    |
---
2019-08-26T22:27:42.7503950Z    Compiling tokio-tcp v0.1.3
2019-08-26T22:27:45.6521155Z    Compiling tokio-uds v0.2.5
2019-08-26T22:27:46.7645689Z    Compiling tokio-process v0.2.3
2019-08-26T22:27:49.1438162Z    Compiling tokio v0.1.14
2019-08-26T22:27:54.2996031Z warning: use of deprecated item 'build::rustc::rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-08-26T22:27:54.2997271Z    |
2019-08-26T22:27:54.2997754Z 18 | extern crate rustc_plugin;
2019-08-26T22:27:54.2998246Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-26T22:27:54.2998666Z    |
2019-08-26T22:27:54.2998666Z    |
2019-08-26T22:27:54.2999388Z    = note: `#[warn(deprecated)]` on by default
2019-08-26T22:27:54.2999631Z 
2019-08-26T22:28:03.4221240Z warning: use of deprecated item 'build::rustc::rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-08-26T22:28:03.4222800Z    |
2019-08-26T22:28:03.4223298Z 18 | extern crate rustc_plugin;
2019-08-26T22:28:03.4223765Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-26T22:28:03.4224208Z    |
---
2019-08-26T22:38:52.5492807Z Verifying status of clippy-driver...
2019-08-26T22:38:52.5514335Z Verifying status of miri...
2019-08-26T22:38:52.5527837Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-08-26T22:38:52.5539485Z 
2019-08-26T22:38:52.5540208Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-08-26T22:38:52.5540285Z 
2019-08-26T22:38:52.5540614Z If you do intend to update 'miri', please check the error messages above and
2019-08-26T22:38:52.5540670Z commit another update.
2019-08-26T22:38:52.5540700Z 
2019-08-26T22:38:52.5540982Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-08-26T22:38:52.5541283Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-08-26T22:38:52.5541335Z proper steps.
2019-08-26T22:38:52.5569889Z   local time: Mon Aug 26 22:38:52 UTC 2019
2019-08-26T22:38:52.7134130Z   network time: Mon, 26 Aug 2019 22:38:52 GMT
2019-08-26T22:38:52.7134287Z == end clock drift check ==
2019-08-26T22:38:52.7134287Z == end clock drift check ==
2019-08-26T22:38:53.6542548Z ##[error]Bash exited with code '3'.
2019-08-26T22:38:53.6608837Z ##[section]Starting: Checkout
2019-08-26T22:38:53.6610654Z ==============================================================================
2019-08-26T22:38:53.6610731Z Task         : Get sources
2019-08-26T22:38:53.6610784Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
