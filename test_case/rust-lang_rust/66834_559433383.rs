plain
2019-11-28T10:16:35.5938549Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T10:16:35.5953353Z ##[command]git config gc.auto 0
2019-11-28T10:16:35.5957117Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T10:16:35.5960445Z ##[command]git config --get-all http.proxy
2019-11-28T10:16:35.5964501Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66834/merge:refs/remotes/pull/66834/merge
---
2019-11-28T10:19:51.7647569Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-28T10:19:51.7666280Z Traceback (most recent call last):
2019-11-28T10:19:51.7666357Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:51.7666405Z     main()
2019-11-28T10:19:51.7666500Z   File "/checkout/src/bootstrap/bootstrap.py", line 910, in main
2019-11-28T10:19:51.7666550Z     bootstrap(help_triggered)
2019-11-28T10:19:51.7666598Z   File "/checkout/src/bootstrap/bootstrap.py", line 881, in bootstrap
2019-11-28T10:19:51.7666645Z     build.build_bootstrap()
2019-11-28T10:19:51.7666741Z   File "/checkout/src/bootstrap/bootstrap.py", line 646, in build_bootstrap
2019-11-28T10:19:51.7667193Z     env["RUSTFLAGS"] += " -Cdebuginfo=2"
2019-11-28T10:19:51.7667424Z KeyError: 'RUSTFLAGS'
2019-11-28T10:19:51.7713154Z Makefile:67: recipe for target 'prepare' failed
2019-11-28T10:19:52.7733465Z Command failed. Attempt 2/5:
2019-11-28T10:19:52.8113424Z Traceback (most recent call last):
2019-11-28T10:19:52.8114503Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:52.8114503Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:52.8115182Z     main()
2019-11-28T10:19:52.8115601Z   File "/checkout/src/bootstrap/bootstrap.py", line 910, in main
2019-11-28T10:19:52.8115958Z     bootstrap(help_triggered)
2019-11-28T10:19:52.8116033Z   File "/checkout/src/bootstrap/bootstrap.py", line 881, in bootstrap
2019-11-28T10:19:52.8119871Z     build.build_bootstrap()
2019-11-28T10:19:52.8120036Z   File "/checkout/src/bootstrap/bootstrap.py", line 646, in build_bootstrap
2019-11-28T10:19:52.8122649Z     env["RUSTFLAGS"] += " -Cdebuginfo=2"
2019-11-28T10:19:52.8122959Z KeyError: 'RUSTFLAGS'
2019-11-28T10:19:52.8153382Z make: *** [prepare] Error 1
2019-11-28T10:19:54.8169282Z Command failed. Attempt 3/5:
2019-11-28T10:19:54.8596667Z Traceback (most recent call last):
2019-11-28T10:19:54.8596887Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:54.8596887Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:54.8596936Z     main()
2019-11-28T10:19:54.8596981Z   File "/checkout/src/bootstrap/bootstrap.py", line 910, in main
2019-11-28T10:19:54.8602198Z     bootstrap(help_triggered)
2019-11-28T10:19:54.8602859Z   File "/checkout/src/bootstrap/bootstrap.py", line 881, in bootstrap
2019-11-28T10:19:54.8603102Z     build.build_bootstrap()
2019-11-28T10:19:54.8603443Z   File "/checkout/src/bootstrap/bootstrap.py", line 646, in build_bootstrap
2019-11-28T10:19:54.8607192Z     env["RUSTFLAGS"] += " -Cdebuginfo=2"
2019-11-28T10:19:54.8607815Z KeyError: 'RUSTFLAGS'
2019-11-28T10:19:54.8638491Z make: *** [prepare] Error 1
2019-11-28T10:19:57.8655841Z Command failed. Attempt 4/5:
2019-11-28T10:19:57.9026927Z Traceback (most recent call last):
2019-11-28T10:19:57.9027968Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:57.9027968Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:19:57.9030639Z     main()
2019-11-28T10:19:57.9030845Z   File "/checkout/src/bootstrap/bootstrap.py", line 910, in main
2019-11-28T10:19:57.9031223Z     bootstrap(help_triggered)
2019-11-28T10:19:57.9031361Z   File "/checkout/src/bootstrap/bootstrap.py", line 881, in bootstrap
2019-11-28T10:19:57.9034267Z     build.build_bootstrap()
2019-11-28T10:19:57.9034481Z   File "/checkout/src/bootstrap/bootstrap.py", line 646, in build_bootstrap
2019-11-28T10:19:57.9035257Z     env["RUSTFLAGS"] += " -Cdebuginfo=2"
2019-11-28T10:19:57.9035744Z KeyError: 'RUSTFLAGS'
2019-11-28T10:19:57.9060993Z make: *** [prepare] Error 1
2019-11-28T10:20:01.9079527Z Command failed. Attempt 5/5:
2019-11-28T10:20:01.9500743Z Traceback (most recent call last):
2019-11-28T10:20:01.9501503Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:20:01.9501503Z   File "/checkout/src/bootstrap/bootstrap.py", line 927, in <module>
2019-11-28T10:20:01.9501838Z     main()
2019-11-28T10:20:01.9502066Z   File "/checkout/src/bootstrap/bootstrap.py", line 910, in main
2019-11-28T10:20:01.9505062Z     bootstrap(help_triggered)
2019-11-28T10:20:01.9505388Z   File "/checkout/src/bootstrap/bootstrap.py", line 881, in bootstrap
2019-11-28T10:20:01.9509261Z     build.build_bootstrap()
2019-11-28T10:20:01.9509655Z   File "/checkout/src/bootstrap/bootstrap.py", line 646, in build_bootstrap
2019-11-28T10:20:01.9510532Z     env["RUSTFLAGS"] += " -Cdebuginfo=2"
2019-11-28T10:20:01.9511044Z KeyError: 'RUSTFLAGS'
2019-11-28T10:20:01.9539619Z make: *** [prepare] Error 1
2019-11-28T10:20:01.9543071Z The command has failed after 5 attempts.
2019-11-28T10:20:01.9543634Z == clock drift check ==
2019-11-28T10:20:01.9550285Z   local time: Thu Nov 28 10:20:01 UTC 2019
2019-11-28T10:20:01.9550285Z   local time: Thu Nov 28 10:20:01 UTC 2019
2019-11-28T10:20:02.2244561Z   network time: Thu, 28 Nov 2019 10:20:02 GMT
2019-11-28T10:20:02.2246075Z == end clock drift check ==
2019-11-28T10:20:13.6150519Z 
2019-11-28T10:20:13.6245811Z ##[error]Bash exited with code '1'.
2019-11-28T10:20:13.6282970Z ##[section]Starting: Checkout
2019-11-28T10:20:13.6308149Z ==============================================================================
2019-11-28T10:20:13.6308196Z Task         : Get sources
2019-11-28T10:20:13.6308263Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
