plain
2020-03-09T17:52:12.4545021Z Prepare build directory.
2020-03-09T17:52:12.4882795Z Set build variables.
2020-03-09T17:52:12.4918587Z Download all required tasks.
2020-03-09T17:52:12.5039437Z Downloading task: Bash (3.163.1)
2020-03-09T17:52:13.5514503Z Checking job knob settings.
2020-03-09T17:52:13.5537329Z Finished checking job knob settings.
2020-03-09T17:52:13.6074923Z ##[section]Finishing: Initialize job
2020-03-09T17:52:13.6402526Z ##[section]Starting: Configure Job Name
2020-03-09T17:52:13.6609215Z ==============================================================================
2020-03-09T17:52:13.6609901Z Task         : Bash
---
2020-03-09T17:52:14.6932777Z ========================== Starting Command Output ===========================
2020-03-09T17:52:14.6935071Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/99cb2540-a66c-4f6f-b255-86df867a139f.sh
2020-03-09T17:52:14.6935299Z 
2020-03-09T17:52:14.6939407Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T17:52:14.6959168Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-09T17:52:14.6962458Z Task         : Get sources
2020-03-09T17:52:14.6962713Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T17:52:14.6962959Z Version      : 1.0.0
2020-03-09T17:52:14.6963170Z Author       : Microsoft
---
2020-03-09T17:52:16.0065055Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T17:52:16.0071415Z ##[command]git config gc.auto 0
2020-03-09T17:52:16.0074580Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T17:52:16.0078301Z ##[command]git config --get-all http.proxy
2020-03-09T17:52:16.0085305Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69859/merge:refs/remotes/pull/69859/merge
---
2020-03-09T17:57:50.3141741Z Done!
2020-03-09T17:57:50.3142017Z some tidy checks failed
2020-03-09T17:57:50.3142329Z 
2020-03-09T17:57:50.3142496Z 
2020-03-09T17:57:50.3143701Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-09T17:57:50.3154079Z 
2020-03-09T17:57:50.3154427Z 
2020-03-09T17:57:50.3154900Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-09T17:57:50.3155405Z Build completed unsuccessfully in 0:01:28
2020-03-09T17:57:50.3155405Z Build completed unsuccessfully in 0:01:28
2020-03-09T17:57:50.3202451Z == clock drift check ==
2020-03-09T17:57:50.3211883Z   local time: Mon Mar  9 17:57:50 UTC 2020
2020-03-09T17:57:50.5873868Z   network time: Mon, 09 Mar 2020 17:57:50 GMT
2020-03-09T17:57:50.5878215Z == end clock drift check ==
2020-03-09T17:57:51.3837889Z 
2020-03-09T17:57:51.3872397Z ##[error]Bash exited with code '1'.
2020-03-09T17:57:51.3886117Z ##[section]Finishing: Run build
2020-03-09T17:57:51.3930438Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-09T17:57:51.3936005Z Task         : Get sources
2020-03-09T17:57:51.3936370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T17:57:51.3936728Z Version      : 1.0.0
2020-03-09T17:57:51.3936957Z Author       : Microsoft
2020-03-09T17:57:51.3936957Z Author       : Microsoft
2020-03-09T17:57:51.3937321Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-09T17:57:51.3937765Z ==============================================================================
2020-03-09T17:57:51.6975456Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-09T17:57:51.7015292Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-09T17:57:51.7111197Z Cleaning up task key
2020-03-09T17:57:51.7112608Z Start cleaning up orphan processes.
2020-03-09T17:57:51.7299335Z Terminate orphan process: pid (4600) (python)
2020-03-09T17:57:51.7441848Z ##[section]Finishing: Finalize Job
