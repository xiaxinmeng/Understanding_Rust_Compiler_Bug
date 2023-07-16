plain
2020-02-22T11:55:46.8239993Z ========================== Starting Command Output ===========================
2020-02-22T11:55:46.8244765Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/89ce69e5-f1f9-4faa-b75d-e875c813694f.sh
2020-02-22T11:55:46.8245289Z 
2020-02-22T11:55:46.8250165Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T11:55:46.8272236Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69372/merge to s
2020-02-22T11:55:46.8276413Z Task         : Get sources
2020-02-22T11:55:46.8276645Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T11:55:46.8276924Z Version      : 1.0.0
2020-02-22T11:55:46.8277076Z Author       : Microsoft
---
2020-02-22T11:55:49.4501304Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T11:55:49.4650113Z ##[command]git config gc.auto 0
2020-02-22T11:55:49.4692230Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T11:55:49.4732383Z ##[command]git config --get-all http.proxy
2020-02-22T11:55:49.4828837Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69372/merge:refs/remotes/pull/69372/merge
---
2020-02-22T12:02:58.5557131Z    Compiling serde_json v1.0.40
2020-02-22T12:03:00.1652037Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-22T12:03:10.3002607Z     Finished release [optimized] target(s) in 1m 29s
2020-02-22T12:03:10.3092777Z tidy check
2020-02-22T12:03:10.5243979Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0364.md:29: trailing whitespace
2020-02-22T12:03:10.5250790Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0364.md:32: line longer than 80 chars
2020-02-22T12:03:10.5251314Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0303.md: too many trailing newlines (2)
2020-02-22T12:03:10.5251857Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0601.md:11: trailing whitespace
2020-02-22T12:03:10.5259306Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0535.md:30: line longer than 80 chars
2020-02-22T12:03:10.5259827Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0382.md:107: trailing whitespace
2020-02-22T12:03:10.5260348Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0382.md:110: line longer than 80 chars
2020-02-22T12:03:10.5277780Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0503.md:51: trailing whitespace
2020-02-22T12:03:10.5278312Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0503.md:54: line longer than 80 chars
2020-02-22T12:03:10.5278802Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0507.md:130: trailing whitespace
2020-02-22T12:03:10.5279295Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0507.md:133: line longer than 80 chars
2020-02-22T12:03:10.5286908Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0502.md:28: trailing whitespace
2020-02-22T12:03:10.5287414Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0502.md:31: line longer than 80 chars
2020-02-22T12:03:10.5299592Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0455.md:21: line longer than 80 chars
2020-02-22T12:03:10.5306298Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0154.md:30: trailing whitespace
2020-02-22T12:03:10.5306989Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0154.md:31: trailing whitespace
2020-02-22T12:03:10.5307539Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0154.md:34: line longer than 80 chars
2020-02-22T12:03:10.5311000Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0505.md:84: trailing whitespace
2020-02-22T12:03:10.5311501Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0505.md:87: line longer than 80 chars
2020-02-22T12:03:10.5312196Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0080.md:22: line longer than 80 chars
2020-02-22T12:03:10.5312710Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0080.md:23: line longer than 80 chars
2020-02-22T12:03:10.5313318Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0260.md:31: trailing whitespace
2020-02-22T12:03:10.5317660Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0260.md:32: trailing whitespace
2020-02-22T12:03:10.5318158Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0260.md:35: line longer than 80 chars
2020-02-22T12:03:10.5318644Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0661.md:10: trailing whitespace
2020-02-22T12:03:10.5319138Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0662.md:12: trailing whitespace
2020-02-22T12:03:10.5322667Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0499.md:17: line longer than 80 chars
2020-02-22T12:03:10.5323227Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0663.md:12: trailing whitespace
2020-02-22T12:03:10.5323744Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0534.md:37: line longer than 80 chars
2020-02-22T12:03:10.5329888Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0525.md:39: trailing whitespace
2020-02-22T12:03:10.5336636Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0660.md:9: trailing whitespace
2020-02-22T12:03:10.5337119Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0501.md:9: line longer than 80 chars
2020-02-22T12:03:10.5352812Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0365.md:29: trailing whitespace
2020-02-22T12:03:10.5353330Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0365.md:32: line longer than 80 chars
2020-02-22T12:03:10.5358460Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0536.md:21: trailing whitespace
2020-02-22T12:03:10.5358988Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0536.md:24: line longer than 80 chars
2020-02-22T12:03:10.5359525Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0537.md:27: trailing whitespace
2020-02-22T12:03:10.5360014Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0537.md:30: line longer than 80 chars
2020-02-22T12:03:10.5363634Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0664.md:13: trailing whitespace
2020-02-22T12:03:12.9789555Z Found 487 error codes
2020-02-22T12:03:12.9789848Z Found 0 error codes with no tests
2020-02-22T12:03:12.9790084Z Done!
2020-02-22T12:03:12.9790257Z some tidy checks failed
2020-02-22T12:03:12.9790257Z some tidy checks failed
2020-02-22T12:03:12.9790405Z 
2020-02-22T12:03:12.9790508Z 
2020-02-22T12:03:12.9791866Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-22T12:03:12.9792661Z 
2020-02-22T12:03:12.9792812Z 
2020-02-22T12:03:12.9797244Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-22T12:03:12.9797987Z Build completed unsuccessfully in 0:01:39
2020-02-22T12:03:12.9797987Z Build completed unsuccessfully in 0:01:39
2020-02-22T12:03:12.9843607Z == clock drift check ==
2020-02-22T12:03:12.9873529Z   local time: Sat Feb 22 12:03:12 UTC 2020
2020-02-22T12:03:13.2703960Z   network time: Sat, 22 Feb 2020 12:03:13 GMT
2020-02-22T12:03:13.2708704Z == end clock drift check ==
2020-02-22T12:03:14.0476128Z 
2020-02-22T12:03:14.0553155Z ##[error]Bash exited with code '1'.
2020-02-22T12:03:14.0566880Z ##[section]Finishing: Run build
2020-02-22T12:03:14.0611183Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69372/merge to s
2020-02-22T12:03:14.0616139Z Task         : Get sources
2020-02-22T12:03:14.0616682Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T12:03:14.0617033Z Version      : 1.0.0
2020-02-22T12:03:14.0617290Z Author       : Microsoft
2020-02-22T12:03:14.0617290Z Author       : Microsoft
2020-02-22T12:03:14.0617673Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T12:03:14.0618302Z ==============================================================================
2020-02-22T12:03:14.3960268Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T12:03:14.4005613Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69372/merge to s
2020-02-22T12:03:14.4096926Z Cleaning up task key
2020-02-22T12:03:14.4098392Z Start cleaning up orphan processes.
2020-02-22T12:03:14.4397977Z Terminate orphan process: pid (3999) (python)
2020-02-22T12:03:14.4434585Z ##[section]Finishing: Finalize Job
