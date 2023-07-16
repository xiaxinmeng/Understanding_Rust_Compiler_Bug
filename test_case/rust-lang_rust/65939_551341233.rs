plain
2019-11-08T01:17:14.3726944Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T01:17:14.3933442Z ##[command]git config gc.auto 0
2019-11-08T01:17:14.3991666Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T01:17:14.4039193Z ##[command]git config --get-all http.proxy
2019-11-08T01:17:14.4176835Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65939/merge:refs/remotes/pull/65939/merge
---
2019-11-08T01:20:40.1899116Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:40.1917238Z 
2019-11-08T01:20:40.1917362Z spurious failure, trying again
2019-11-08T01:20:40.4262139Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:40.4284583Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpJrM7Gj.sha256 https://static.rust-lang.org/dist/2019-09-25/rustfmt-nightly-2019-11-05-x86_64-unknown-linux-gnu.tar.gz.sha256
2019-11-08T01:20:40.4329014Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T01:20:40.4331647Z make: *** [prepare] Error 1
2019-11-08T01:20:41.4356748Z Command failed. Attempt 2/5:
2019-11-08T01:20:41.6116685Z curl: (22) The requested URL returned error: 404 Not Found
---
2019-11-08T01:20:42.0124220Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:42.0142230Z 
2019-11-08T01:20:42.0142619Z spurious failure, trying again
2019-11-08T01:20:42.1449861Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:42.1466238Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpgOOFww.sha256 https://static.rust-lang.org/dist/2019-09-25/rustfmt-nightly-2019-11-05-x86_64-unknown-linux-gnu.tar.gz.sha256
2019-11-08T01:20:42.1519799Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T01:20:42.1519890Z make: *** [prepare] Error 1
2019-11-08T01:20:44.1538906Z Command failed. Attempt 3/5:
2019-11-08T01:20:44.3458326Z curl: (22) The requested URL returned error: 404 Not Found
---
2019-11-08T01:20:44.7322210Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:44.7333236Z 
2019-11-08T01:20:44.7334449Z spurious failure, trying again
2019-11-08T01:20:44.8675362Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:44.8693446Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpdk_iVJ.sha256 https://static.rust-lang.org/dist/2019-09-25/rustfmt-nightly-2019-11-05-x86_64-unknown-linux-gnu.tar.gz.sha256
2019-11-08T01:20:44.8745848Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T01:20:44.8746325Z make: *** [prepare] Error 1
2019-11-08T01:20:47.8771952Z Command failed. Attempt 4/5:
2019-11-08T01:20:48.0371050Z curl: (22) The requested URL returned error: 404 Not Found
---
2019-11-08T01:20:48.4493999Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:48.4509634Z 
2019-11-08T01:20:48.4509753Z spurious failure, trying again
2019-11-08T01:20:48.6464994Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:48.6484188Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpj0U9Qt.sha256 https://static.rust-lang.org/dist/2019-09-25/rustfmt-nightly-2019-11-05-x86_64-unknown-linux-gnu.tar.gz.sha256
2019-11-08T01:20:48.6538995Z make: *** [prepare] Error 1
2019-11-08T01:20:48.6539485Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T01:20:52.6562345Z Command failed. Attempt 5/5:
2019-11-08T01:20:52.8177887Z curl: (22) The requested URL returned error: 404 Not Found
---
2019-11-08T01:20:53.3438660Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:53.3453075Z 
2019-11-08T01:20:53.3453622Z spurious failure, trying again
2019-11-08T01:20:53.4578041Z curl: (22) The requested URL returned error: 404 Not Found
2019-11-08T01:20:53.4598418Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpoHfDxs.sha256 https://static.rust-lang.org/dist/2019-09-25/rustfmt-nightly-2019-11-05-x86_64-unknown-linux-gnu.tar.gz.sha256
2019-11-08T01:20:53.4645410Z Makefile:67: recipe for target 'prepare' failed
2019-11-08T01:20:53.4646007Z make: *** [prepare] Error 1
2019-11-08T01:20:53.4651596Z The command has failed after 5 attempts.
2019-11-08T01:20:53.4654395Z == clock drift check ==
2019-11-08T01:20:53.4654395Z == clock drift check ==
2019-11-08T01:20:53.4668548Z   local time: Fri Nov  8 01:20:53 UTC 2019
2019-11-08T01:20:53.4899859Z   network time: Fri, 08 Nov 2019 01:20:53 GMT
2019-11-08T01:20:53.4902342Z == end clock drift check ==
2019-11-08T01:21:07.1284674Z 
2019-11-08T01:21:07.1391852Z ##[error]Bash exited with code '1'.
2019-11-08T01:21:07.1418327Z ##[section]Starting: Checkout
2019-11-08T01:21:07.1420171Z ==============================================================================
2019-11-08T01:21:07.1420247Z Task         : Get sources
2019-11-08T01:21:07.1420296Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
