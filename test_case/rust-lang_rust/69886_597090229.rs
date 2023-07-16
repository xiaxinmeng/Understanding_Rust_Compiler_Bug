plain
2020-03-10T13:36:23.9794072Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:23.9804276Z 
2020-03-10T13:36:23.9804844Z spurious failure, trying again
2020-03-10T13:36:24.1605519Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:24.1626603Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpIvcMPd.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T13:36:24.1686400Z make: *** [prepare] Error 1
2020-03-10T13:36:25.1705530Z Command failed. Attempt 2/5:
2020-03-10T13:36:25.3860353Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:25.3873601Z 
---
2020-03-10T13:36:25.8736632Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:25.8750068Z 
2020-03-10T13:36:25.8750602Z spurious failure, trying again
2020-03-10T13:36:26.0407909Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:26.0426363Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpJsJznv.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T13:36:26.0467250Z make: *** [prepare] Error 1
2020-03-10T13:36:28.0483909Z Command failed. Attempt 3/5:
2020-03-10T13:36:28.2627428Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:28.2639929Z 
---
2020-03-10T13:36:28.8618270Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:28.8635479Z 
2020-03-10T13:36:28.8635743Z spurious failure, trying again
2020-03-10T13:36:29.0076286Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:29.0097250Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpiOcG2I.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T13:36:29.0140810Z make: *** [prepare] Error 1
2020-03-10T13:36:32.0162030Z Command failed. Attempt 4/5:
2020-03-10T13:36:32.2280415Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:32.2292034Z 
---
2020-03-10T13:36:32.6772978Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:32.6785106Z 
2020-03-10T13:36:32.8181247Z spurious failure, trying again
2020-03-10T13:36:32.8181788Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:32.8200300Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpFvfsnq.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T13:36:32.8245031Z make: *** [prepare] Error 1
2020-03-10T13:36:36.8263720Z Command failed. Attempt 5/5:
2020-03-10T13:36:37.0400757Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:37.0414935Z 
---
2020-03-10T13:36:37.4795728Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:37.4830914Z 
2020-03-10T13:36:37.4831538Z spurious failure, trying again
2020-03-10T13:36:37.6494540Z curl: (22) The requested URL returned error: 404 Not Found
2020-03-10T13:36:37.6517467Z failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmptHLlFF.sha256 https://dev-static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
2020-03-10T13:36:37.6562977Z make: *** [prepare] Error 1
2020-03-10T13:36:37.6563282Z The command has failed after 5 attempts.
2020-03-10T13:36:37.6563525Z == clock drift check ==
2020-03-10T13:36:37.6571913Z   local time: Tue Mar 10 13:36:37 UTC 2020
2020-03-10T13:36:37.6571913Z   local time: Tue Mar 10 13:36:37 UTC 2020
2020-03-10T13:36:37.9454297Z   network time: Tue, 10 Mar 2020 13:36:37 GMT
2020-03-10T13:36:37.9454865Z == end clock drift check ==
2020-03-10T13:36:45.1788183Z 
2020-03-10T13:36:45.1866796Z ##[error]Bash exited with code '1'.
2020-03-10T13:36:45.1926843Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-10T13:36:45.1931357Z ==============================================================================
2020-03-10T13:36:45.1931688Z Task         : Get sources
2020-03-10T13:36:45.1932043Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
