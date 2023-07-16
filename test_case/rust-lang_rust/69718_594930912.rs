plain
2020-03-04T23:12:46.5658145Z ========================== Starting Command Output ===========================
2020-03-04T23:12:46.5661601Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4b0fcbe7-51f6-4a7d-972b-8b1ccf40408d.sh
2020-03-04T23:12:46.5662020Z 
2020-03-04T23:12:46.5665831Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-04T23:12:46.5684604Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-04T23:12:46.5687664Z Task         : Get sources
2020-03-04T23:12:46.5687971Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T23:12:46.5688254Z Version      : 1.0.0
2020-03-04T23:12:46.5688442Z Author       : Microsoft
---
2020-03-04T23:12:47.8219429Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-04T23:12:47.8229059Z ##[command]git config gc.auto 0
2020-03-04T23:12:47.8235424Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-04T23:12:47.8241362Z ##[command]git config --get-all http.proxy
2020-03-04T23:12:47.8252896Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69718/merge:refs/remotes/pull/69718/merge
---
2020-03-04T23:19:15.2142554Z    Compiling serde_json v1.0.40
2020-03-04T23:19:16.5418764Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-04T23:19:25.0938143Z     Finished release [optimized] target(s) in 1m 07s
2020-03-04T23:19:25.0942376Z tidy check
2020-03-04T23:19:25.5104497Z tidy error: /checkout/src/test/codegen/src-file-checksum/src-file-checksum-md5.rs: missing trailing newline
2020-03-04T23:19:27.2614091Z * block-buffer 
2020-03-04T23:19:27.2614660Z * block-padding 
2020-03-04T23:19:27.2615198Z * byte-tools 
2020-03-04T23:19:27.2615937Z * digest 
---
2020-03-04T23:19:27.5012872Z Done!
2020-03-04T23:19:27.5013037Z some tidy checks failed
2020-03-04T23:19:27.5013221Z 
2020-03-04T23:19:27.5013323Z 
2020-03-04T23:19:27.5014585Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-04T23:19:27.5015352Z 
2020-03-04T23:19:27.5015648Z 
2020-03-04T23:19:27.5015988Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-04T23:19:27.5016359Z Build completed unsuccessfully in 0:01:17
2020-03-04T23:19:27.5016359Z Build completed unsuccessfully in 0:01:17
2020-03-04T23:19:27.5016611Z == clock drift check ==
2020-03-04T23:19:27.5023520Z   local time: Wed Mar  4 23:19:27 UTC 2020
2020-03-04T23:19:27.7695975Z   network time: Wed, 04 Mar 2020 23:19:27 GMT
2020-03-04T23:19:27.7703897Z == end clock drift check ==
2020-03-04T23:19:28.5774808Z 
2020-03-04T23:19:28.5848004Z ##[error]Bash exited with code '1'.
2020-03-04T23:19:28.5881347Z ##[section]Finishing: Run build
2020-03-04T23:19:28.5925271Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-04T23:19:28.5929949Z Task         : Get sources
2020-03-04T23:19:28.5930413Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T23:19:28.5930708Z Version      : 1.0.0
2020-03-04T23:19:28.5930914Z Author       : Microsoft
2020-03-04T23:19:28.5930914Z Author       : Microsoft
2020-03-04T23:19:28.5931265Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-04T23:19:28.5931643Z ==============================================================================
2020-03-04T23:19:28.9059937Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-04T23:19:28.9099929Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-04T23:19:28.9216370Z Cleaning up task key
2020-03-04T23:19:28.9217581Z Start cleaning up orphan processes.
2020-03-04T23:19:28.9477324Z Terminate orphan process: pid (20886) (python)
2020-03-04T23:19:28.9503704Z ##[section]Finishing: Finalize Job
