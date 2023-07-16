plain
2020-04-16T22:41:34.2284043Z ========================== Starting Command Output ===========================
2020-04-16T22:41:34.2288399Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/13c6f742-4b5e-4d5a-860d-b6980a66077d.sh
2020-04-16T22:41:34.2289402Z 
2020-04-16T22:41:34.2296453Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T22:41:34.2314139Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-16T22:41:34.2317246Z Task         : Get sources
2020-04-16T22:41:34.2317545Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T22:41:34.2317822Z Version      : 1.0.0
2020-04-16T22:41:34.2318013Z Author       : Microsoft
---
2020-04-16T22:41:35.5007418Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T22:41:35.5014881Z ##[command]git config gc.auto 0
2020-04-16T22:41:35.5019174Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T22:41:35.5023099Z ##[command]git config --get-all http.proxy
2020-04-16T22:41:35.5035244Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-16T22:44:06.2269587Z  ---> 78ad2f4d4aca
2020-04-16T22:44:06.2269804Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-16T22:44:06.2289947Z  ---> Using cache
2020-04-16T22:44:06.2290355Z  ---> 4d2dc61c4d00
2020-04-16T22:44:06.2291589Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-16T22:44:06.2292815Z  ---> 776b6266a8b7
2020-04-16T22:44:06.2309815Z Successfully built 776b6266a8b7
2020-04-16T22:44:06.2334532Z Successfully tagged rust-ci:latest
2020-04-16T22:44:06.4070958Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T22:44:06.4070958Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T22:44:06.4086538Z Looks like docker image is the same as before, not uploading
2020-04-16T22:44:13.6096009Z [CI_JOB_NAME=mingw-check]
2020-04-16T22:44:13.6333466Z [CI_JOB_NAME=mingw-check]
2020-04-16T22:44:13.6361408Z == clock drift check ==
2020-04-16T22:44:13.6367808Z   local time: Thu Apr 16 22:44:13 UTC 2020
2020-04-16T22:44:13.7005993Z   network time: Thu, 16 Apr 2020 22:44:13 GMT
2020-04-16T22:44:13.7027542Z Starting sccache server...
2020-04-16T22:44:13.8146616Z configure: processing command line
2020-04-16T22:44:13.8147225Z configure: 
2020-04-16T22:44:13.8148301Z configure: rust.parallel-compiler := True
---
2020-04-16T22:47:58.0491024Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T22:47:58.1314033Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T22:47:58.3141588Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T22:47:58.4390423Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T22:47:58.8757350Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T22:48:01.0040527Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T22:48:01.4592183Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T22:48:03.4273961Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T22:48:03.8456674Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T22:49:46.6949897Z configure: rust.dist-src        := False
2020-04-16T22:49:46.6950179Z configure: llvm.ccache          := sccache
2020-04-16T22:49:46.6950628Z configure: rust.debug-assertions := True
2020-04-16T22:49:46.6950894Z configure: rust.channel         := nightly
2020-04-16T22:49:46.6951451Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-16T22:49:46.6951944Z configure: writing `config.toml` in current directory
2020-04-16T22:49:46.6952173Z configure: 
2020-04-16T22:49:46.6952551Z configure: run `python /checkout/x.py --help`
2020-04-16T22:49:46.6952756Z configure: 
---
2020-04-16T22:51:18.8743225Z Hugepagesize:       2048 kB
2020-04-16T22:51:18.8743425Z DirectMap4k:      143296 kB
2020-04-16T22:51:18.8743642Z DirectMap2M:     3002368 kB
2020-04-16T22:51:18.8743840Z DirectMap1G:     6291456 kB
2020-04-16T22:51:18.8812690Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-16T22:51:20.1949912Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-16T22:51:20.1949912Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-16T22:51:20.1958767Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-16T22:51:20.4187355Z    Compiling unicode-xid v0.2.0
2020-04-16T22:51:20.5432897Z    Compiling syn v1.0.11
2020-04-16T22:51:21.3918802Z    Compiling linked-hash-map v0.5.2
2020-04-16T22:51:21.3995935Z    Compiling lazy_static v1.4.0
2020-04-16T22:51:21.3995935Z    Compiling lazy_static v1.4.0
2020-04-16T22:51:21.6242314Z    Compiling yaml-rust v0.4.3
2020-04-16T22:51:25.8001310Z    Compiling quote v1.0.2
2020-04-16T22:51:40.4438330Z    Compiling thiserror-impl v1.0.5
2020-04-16T22:51:45.4397520Z    Compiling thiserror v1.0.5
2020-04-16T22:51:45.5013169Z    Compiling yaml-merge-keys v0.4.0
2020-04-16T22:51:46.6673480Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-16T22:51:48.6126066Z Build completed successfully in 0:00:29
2020-04-16T22:51:48.6219275Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-16T22:51:48.8805963Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-16T22:51:50.0218299Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-16T22:53:52.6812916Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T22:53:52.8836616Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T22:53:53.1303374Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T22:53:53.2337647Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T22:53:53.7444104Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T22:53:55.9830009Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T22:53:56.4582695Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T22:53:58.4779964Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T22:53:58.9158819Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T22:57:26.4400252Z    Compiling cargo_metadata v0.9.1
2020-04-16T22:57:30.6910099Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-16T22:57:40.7761965Z     Finished release [optimized] target(s) in 23.83s
2020-04-16T22:57:40.7855552Z tidy check
2020-04-16T22:57:41.4826531Z tidy error: /checkout/src/librustc_trait_selection/traits/select.rs:496: TODO is deprecated; use FIXME
2020-04-16T22:57:41.4909889Z tidy error: /checkout/src/librustc_trait_selection/traits/fulfill.rs:503: TODO is deprecated; use FIXME
2020-04-16T22:57:44.7362491Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:12: trailing whitespace
2020-04-16T22:57:44.7363185Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:25: trailing whitespace
2020-04-16T22:57:44.7363939Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:29: trailing whitespace
2020-04-16T22:57:44.7364608Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:34: trailing whitespace
2020-04-16T22:57:44.7365197Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:43: trailing whitespace
2020-04-16T22:57:49.1669505Z some tidy checks failed
2020-04-16T22:57:49.1678517Z Found 490 error codes
2020-04-16T22:57:49.1680388Z Found 0 error codes with no tests
2020-04-16T22:57:49.1680730Z Done!
2020-04-16T22:57:49.1680730Z Done!
2020-04-16T22:57:49.1680977Z 
2020-04-16T22:57:49.1681141Z 
2020-04-16T22:57:49.1683123Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-16T22:57:49.1684341Z 
2020-04-16T22:57:49.1684505Z 
2020-04-16T22:57:49.1692979Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-16T22:57:49.1693625Z Build completed unsuccessfully in 0:00:33
2020-04-16T22:57:49.1693625Z Build completed unsuccessfully in 0:00:33
2020-04-16T22:57:49.1795535Z == clock drift check ==
2020-04-16T22:57:49.1827771Z   local time: Thu Apr 16 22:57:49 UTC 2020
2020-04-16T22:57:49.5004531Z   network time: Thu, 16 Apr 2020 22:57:49 GMT
2020-04-16T22:57:51.3614991Z 
2020-04-16T22:57:51.3614991Z 
2020-04-16T22:57:51.3686639Z ##[error]Bash exited with code '1'.
2020-04-16T22:57:51.3699811Z ##[section]Finishing: Run build
2020-04-16T22:57:51.3743905Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-16T22:57:51.3748717Z Task         : Get sources
2020-04-16T22:57:51.3749024Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T22:57:51.3749324Z Version      : 1.0.0
2020-04-16T22:57:51.3749528Z Author       : Microsoft
2020-04-16T22:57:51.3749528Z Author       : Microsoft
2020-04-16T22:57:51.3749976Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T22:57:51.3750350Z ==============================================================================
2020-04-16T22:57:51.7030247Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T22:57:51.7078154Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-16T22:57:51.7168373Z Cleaning up task key
2020-04-16T22:57:51.7169793Z Start cleaning up orphan processes.
2020-04-16T22:57:51.7348304Z Terminate orphan process: pid (3529) (python)
2020-04-16T22:57:51.7575229Z ##[section]Finishing: Finalize Job
