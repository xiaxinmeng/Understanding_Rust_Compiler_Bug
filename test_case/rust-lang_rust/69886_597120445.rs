plain
2020-03-10T14:35:09.2338237Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:09.2351372Z 
2020-03-10T14:35:09.2351959Z spurious failure, trying again
2020-03-10T14:35:09.3937081Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:09.3954707Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpR5FKkJ.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T14:35:09.4007388Z make: *** [prepare] Error 1
2020-03-10T14:35:10.4027575Z Command failed. Attempt 2/5:
2020-03-10T14:35:10.6233124Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:10.6249529Z 
---
2020-03-10T14:35:11.1147950Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:11.1190913Z 
2020-03-10T14:35:11.1191258Z spurious failure, trying again
2020-03-10T14:35:11.2733445Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:11.2755289Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp2Pll7z.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T14:35:11.2802648Z make: *** [prepare] Error 1
2020-03-10T14:35:13.2826993Z Command failed. Attempt 3/5:
2020-03-10T14:35:13.4752309Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:13.4753261Z 
---
2020-03-10T14:35:13.9202286Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:13.9213117Z 
2020-03-10T14:35:13.9213823Z spurious failure, trying again
2020-03-10T14:35:14.0635897Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:14.0651719Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpZCsu9Y.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T14:35:14.0693355Z make: *** [prepare] Error 1
2020-03-10T14:35:17.0710529Z Command failed. Attempt 4/5:
2020-03-10T14:35:17.2620287Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:17.2633139Z 
---
2020-03-10T14:35:17.7444755Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:17.7465044Z 
2020-03-10T14:35:17.7465320Z spurious failure, trying again
2020-03-10T14:35:17.8916679Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:17.8936190Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpAmDZuL.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T14:35:17.8979805Z make: *** [prepare] Error 1
2020-03-10T14:35:21.9006110Z Command failed. Attempt 5/5:
2020-03-10T14:35:22.1194004Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:22.1207823Z 
---
2020-03-10T14:35:22.5991946Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:22.6008820Z 
2020-03-10T14:35:22.6009155Z spurious failure, trying again
2020-03-10T14:35:22.7519201Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T14:35:22.7545079Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp5fGVq4.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T14:35:22.7595265Z make: *** [prepare] Error 1
2020-03-10T14:35:22.7607232Z The command has failed after 5 attempts.
2020-03-10T14:35:22.7607802Z == clock drift check ==
2020-03-10T14:35:22.7632485Z   local time: Tue Mar 10 14:35:22 UTC 2020
2020-03-10T14:35:22.7632485Z   local time: Tue Mar 10 14:35:22 UTC 2020
2020-03-10T14:35:22.8589441Z   network time: Tue, 10 Mar 2020 14:35:22 GMT
2020-03-10T14:35:22.8590204Z == end clock drift check ==
2020-03-10T14:35:30.1191193Z 
2020-03-10T14:35:30.1285896Z ##[error]Bash exited with code '1'.
2020-03-10T14:35:30.1359466Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-10T14:35:30.1364948Z ==============================================================================
2020-03-10T14:35:30.1365400Z Task         : Get sources
2020-03-10T14:35:30.1365892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
