plain
2020-04-23T18:24:53.4250689Z ========================== Starting Command Output ===========================
2020-04-23T18:24:53.4254472Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/feb831bd-5dd3-4c18-af4c-ee6bcae48ef0.sh
2020-04-23T18:24:53.4254886Z 
2020-04-23T18:24:53.4259068Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T18:24:53.4278087Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71482/merge to s
2020-04-23T18:24:53.4281298Z Task         : Get sources
2020-04-23T18:24:53.4281537Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T18:24:53.4281767Z Version      : 1.0.0
2020-04-23T18:24:53.4282281Z Author       : Microsoft
---
2020-04-23T18:24:54.4157271Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T18:24:54.4162199Z ##[command]git config gc.auto 0
2020-04-23T18:24:54.4165772Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T18:24:54.4168859Z ##[command]git config --get-all http.proxy
2020-04-23T18:24:54.4174847Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71482/merge:refs/remotes/pull/71482/merge
---
2020-04-23T18:27:59.3726383Z  ---> 78ad2f4d4aca
2020-04-23T18:27:59.3726634Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-23T18:27:59.3731252Z  ---> Using cache
2020-04-23T18:27:59.3731609Z  ---> 4d2dc61c4d00
2020-04-23T18:27:59.3732637Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-23T18:27:59.3774877Z  ---> 776b6266a8b7
2020-04-23T18:27:59.3775069Z Successfully built 776b6266a8b7
2020-04-23T18:27:59.3805026Z Successfully tagged rust-ci:latest
2020-04-23T18:27:59.4341420Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T18:27:59.4341420Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T18:27:59.4355188Z Looks like docker image is the same as before, not uploading
2020-04-23T18:28:07.9217748Z [CI_JOB_NAME=mingw-check]
2020-04-23T18:28:07.9449983Z [CI_JOB_NAME=mingw-check]
2020-04-23T18:28:07.9521848Z == clock drift check ==
2020-04-23T18:28:07.9522306Z   local time: Thu Apr 23 18:28:07 UTC 2020
2020-04-23T18:28:08.1075606Z   network time: Thu, 23 Apr 2020 18:28:08 GMT
2020-04-23T18:28:08.1095750Z Starting sccache server...
2020-04-23T18:28:08.2034736Z configure: processing command line
2020-04-23T18:28:08.2034934Z configure: 
2020-04-23T18:28:08.2035690Z configure: rust.parallel-compiler := True
---
2020-04-23T18:29:52.9915667Z    Compiling compiler_builtins v0.1.27
2020-04-23T18:29:53.9490084Z error: field has missing stability attribute
2020-04-23T18:29:53.9491282Z    --> src/libcore/num/mod.rs:182:24
2020-04-23T18:29:53.9492009Z     |
2020-04-23T18:29:53.9492775Z 182 | pub struct Wrapping<T>(pub T);
2020-04-23T18:29:53.9493991Z 
2020-04-23T18:29:53.9502090Z error: field has missing stability attribute
2020-04-23T18:29:53.9503024Z    --> src/libcore/cmp.rs:468:23
2020-04-23T18:29:53.9503702Z     |
2020-04-23T18:29:53.9503702Z     |
2020-04-23T18:29:53.9504442Z 468 | pub struct Reverse<T>(pub T);
2020-04-23T18:29:53.9505913Z 
2020-04-23T18:29:53.9520963Z error: variant has missing stability attribute
2020-04-23T18:29:53.9521849Z    --> src/libcore/ops/range.rs:636:5
2020-04-23T18:29:53.9522527Z     |
---
2020-04-23T18:29:53.9576503Z 252 |     Err(E),
2020-04-23T18:29:53.9577154Z     |         ^
2020-04-23T18:29:53.9577349Z 
2020-04-23T18:29:53.9592573Z error: variant has missing stability attribute
2020-04-23T18:29:53.9593109Z   --> src/libcore/task/poll.rs:13:5
2020-04-23T18:29:53.9593966Z 13 |     Ready(T),
2020-04-23T18:29:53.9594439Z    |     ^^^^^^^^
2020-04-23T18:29:53.9594614Z 
2020-04-23T18:29:53.9595279Z error: field has missing stability attribute
2020-04-23T18:29:53.9595279Z error: field has missing stability attribute
2020-04-23T18:29:53.9595766Z   --> src/libcore/task/poll.rs:13:11
2020-04-23T18:29:53.9596766Z 13 |     Ready(T),
2020-04-23T18:29:53.9597230Z    |           ^
2020-04-23T18:29:53.9597405Z 
2020-04-23T18:29:53.9597812Z error: variant has missing stability attribute
2020-04-23T18:29:53.9597812Z error: variant has missing stability attribute
2020-04-23T18:29:53.9598294Z   --> src/libcore/task/poll.rs:20:5
2020-04-23T18:29:53.9599305Z 20 |     Pending,
2020-04-23T18:29:53.9599771Z    |     ^^^^^^^
2020-04-23T18:29:53.9599943Z 
2020-04-23T18:29:54.4496181Z error: aborting due to 16 previous errors
---
2020-04-23T18:29:54.9124277Z expected success, got: exit code: 101
2020-04-23T18:29:54.9132584Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-23T18:29:54.9132909Z Build completed unsuccessfully in 0:01:46
2020-04-23T18:29:54.9230510Z == clock drift check ==
2020-04-23T18:29:54.9248582Z   local time: Thu Apr 23 18:29:54 UTC 2020
2020-04-23T18:29:55.0854131Z   network time: Thu, 23 Apr 2020 18:29:55 GMT
2020-04-23T18:29:55.8572926Z 
2020-04-23T18:29:55.8572926Z 
2020-04-23T18:29:55.8636913Z ##[error]Bash exited with code '1'.
2020-04-23T18:29:55.8651016Z ##[section]Finishing: Run build
2020-04-23T18:29:55.8695945Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71482/merge to s
2020-04-23T18:29:55.8700838Z Task         : Get sources
2020-04-23T18:29:55.8701137Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T18:29:55.8701416Z Version      : 1.0.0
2020-04-23T18:29:55.8701629Z Author       : Microsoft
2020-04-23T18:29:55.8701629Z Author       : Microsoft
2020-04-23T18:29:55.8702089Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T18:29:55.8702431Z ==============================================================================
2020-04-23T18:29:56.1705509Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T18:29:56.1744309Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71482/merge to s
2020-04-23T18:29:56.1826169Z Cleaning up task key
2020-04-23T18:29:56.1827330Z Start cleaning up orphan processes.
2020-04-23T18:29:56.2132434Z Terminate orphan process: pid (3806) (python)
2020-04-23T18:29:56.2156173Z ##[section]Finishing: Finalize Job
