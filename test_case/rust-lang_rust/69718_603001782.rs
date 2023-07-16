plain
2020-03-24T02:40:44.9442925Z ========================== Starting Command Output ===========================
2020-03-24T02:40:44.9445743Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/774623ad-cab3-4a5c-94bf-c6350d7a7ae2.sh
2020-03-24T02:40:44.9446044Z 
2020-03-24T02:40:44.9449517Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T02:40:44.9470319Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-24T02:40:44.9481104Z Task         : Get sources
2020-03-24T02:40:44.9481388Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T02:40:44.9481718Z Version      : 1.0.0
2020-03-24T02:40:44.9481923Z Author       : Microsoft
---
2020-03-24T02:40:45.9469041Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T02:40:45.9475301Z ##[command]git config gc.auto 0
2020-03-24T02:40:45.9479411Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T02:40:45.9483203Z ##[command]git config --get-all http.proxy
2020-03-24T02:40:45.9489993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69718/merge:refs/remotes/pull/69718/merge
---
2020-03-24T02:49:58.9174681Z configure: build.locked-deps    := True
2020-03-24T02:49:58.9175029Z configure: llvm.ccache          := sccache
2020-03-24T02:49:58.9175581Z configure: build.cargo-native-static := True
2020-03-24T02:49:58.9176130Z configure: dist.missing-tools   := True
2020-03-24T02:49:58.9176797Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-24T02:49:58.9177438Z configure: writing `config.toml` in current directory
2020-03-24T02:49:58.9177706Z configure: 
2020-03-24T02:49:58.9178180Z configure: run `python /checkout/x.py --help`
2020-03-24T02:49:58.9178449Z configure: 
---
2020-03-24T02:58:44.0384604Z Found 0 error codes with no tests
2020-03-24T02:58:44.0384814Z Done!
2020-03-24T02:58:44.0390429Z 
2020-03-24T02:58:44.0390591Z 
2020-03-24T02:58:44.0392284Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-24T02:58:44.0393377Z 
2020-03-24T02:58:44.0393490Z 
2020-03-24T02:58:44.0400483Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-24T02:58:44.0400926Z Build completed unsuccessfully in 0:00:39
2020-03-24T02:58:44.0400926Z Build completed unsuccessfully in 0:00:39
2020-03-24T02:58:44.0465474Z == clock drift check ==
2020-03-24T02:58:44.0478839Z   local time: Tue Mar 24 02:58:44 UTC 2020
2020-03-24T02:58:44.5947460Z   network time: Tue, 24 Mar 2020 02:58:44 GMT
2020-03-24T02:58:44.5949754Z == end clock drift check ==
2020-03-24T02:58:46.1787522Z 
2020-03-24T02:58:46.1865053Z ##[error]Bash exited with code '1'.
2020-03-24T02:58:46.1881580Z ##[section]Finishing: Run build
2020-03-24T02:58:46.1934167Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-24T02:58:46.1939879Z Task         : Get sources
2020-03-24T02:58:46.1940271Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T02:58:46.1940630Z Version      : 1.0.0
2020-03-24T02:58:46.1940902Z Author       : Microsoft
2020-03-24T02:58:46.1940902Z Author       : Microsoft
2020-03-24T02:58:46.1941299Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T02:58:46.1941763Z ==============================================================================
2020-03-24T02:58:46.6108156Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T02:58:46.6157332Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-24T02:58:46.6263502Z Cleaning up task key
2020-03-24T02:58:46.6264758Z Start cleaning up orphan processes.
2020-03-24T02:58:46.6497350Z Terminate orphan process: pid (3434) (python)
2020-03-24T02:58:46.6722982Z ##[section]Finishing: Finalize Job
