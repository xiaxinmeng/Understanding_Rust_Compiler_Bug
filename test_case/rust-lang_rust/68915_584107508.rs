plain
2020-02-10T12:33:13.8508919Z ========================== Starting Command Output ===========================
2020-02-10T12:33:13.8511553Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aa165ab7-07ad-4243-8557-f7e311981708.sh
2020-02-10T12:33:13.8511794Z 
2020-02-10T12:33:13.8517054Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T12:33:13.8525396Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68915/merge to s
2020-02-10T12:33:13.8527003Z Task         : Get sources
2020-02-10T12:33:13.8527039Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T12:33:13.8527121Z Version      : 1.0.0
2020-02-10T12:33:13.8527155Z Author       : Microsoft
---
2020-02-10T12:33:18.0376313Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T12:33:18.9307459Z ##[command]git config gc.auto 0
2020-02-10T12:33:18.9312936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T12:33:18.9316211Z ##[command]git config --get-all http.proxy
2020-02-10T12:33:18.9325617Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68915/merge:refs/remotes/pull/68915/merge
---
2020-02-10T12:39:36.4728816Z    Compiling serde_json v1.0.40
2020-02-10T12:39:38.1407656Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-10T12:39:48.5632803Z     Finished release [optimized] target(s) in 1m 34s
2020-02-10T12:39:48.5743153Z tidy check
2020-02-10T12:39:49.5921458Z tidy error: /checkout/src/libcore/tests/iter.rs: too many lines (3012) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-02-10T12:39:51.2949138Z Found 487 error codes
2020-02-10T12:39:51.2949948Z Found 0 error codes with no tests
2020-02-10T12:39:51.2950280Z Done!
2020-02-10T12:39:51.2950386Z some tidy checks failed
2020-02-10T12:39:51.2950386Z some tidy checks failed
2020-02-10T12:39:51.2954264Z 
2020-02-10T12:39:51.2954571Z 
2020-02-10T12:39:51.2962046Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-10T12:39:51.2966162Z 
2020-02-10T12:39:51.2966330Z 
2020-02-10T12:39:51.3015501Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-10T12:39:51.3015630Z Build completed unsuccessfully in 0:01:44
2020-02-10T12:39:51.3015630Z Build completed unsuccessfully in 0:01:44
2020-02-10T12:39:51.3022491Z == clock drift check ==
2020-02-10T12:39:51.3033675Z   local time: Mon Feb 10 12:39:51 UTC 2020
2020-02-10T12:39:51.8509186Z   network time: Mon, 10 Feb 2020 12:39:51 GMT
2020-02-10T12:39:51.8509275Z == end clock drift check ==
2020-02-10T12:39:52.6497893Z 
2020-02-10T12:39:52.6625090Z ##[error]Bash exited with code '1'.
2020-02-10T12:39:52.6641289Z ##[section]Finishing: Run build
2020-02-10T12:39:52.6659781Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68915/merge to s
2020-02-10T12:39:52.6661912Z Task         : Get sources
2020-02-10T12:39:52.6661959Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T12:39:52.6662024Z Version      : 1.0.0
2020-02-10T12:39:52.6662066Z Author       : Microsoft
2020-02-10T12:39:52.6662066Z Author       : Microsoft
2020-02-10T12:39:52.6662112Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T12:39:52.6662181Z ==============================================================================
2020-02-10T12:39:53.0909843Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T12:39:53.0990401Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68915/merge to s
2020-02-10T12:39:53.1111515Z Cleaning up task key
2020-02-10T12:39:53.1112318Z Start cleaning up orphan processes.
2020-02-10T12:39:53.1232744Z Terminate orphan process: pid (3790) (python)
2020-02-10T12:39:53.1694456Z ##[section]Finishing: Finalize Job
