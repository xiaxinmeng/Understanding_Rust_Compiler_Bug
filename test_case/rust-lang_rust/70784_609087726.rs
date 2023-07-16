plain
2020-04-04T19:04:20.8915281Z ========================== Starting Command Output ===========================
2020-04-04T19:04:20.8921386Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/40cad642-a386-44a2-aa1d-0985b06ac96b.sh
2020-04-04T19:04:20.8921827Z 
2020-04-04T19:04:20.8926774Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T19:04:20.8949300Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70784/merge to s
2020-04-04T19:04:20.8953503Z Task         : Get sources
2020-04-04T19:04:20.8953816Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T19:04:20.8954117Z Version      : 1.0.0
2020-04-04T19:04:20.8954319Z Author       : Microsoft
---
2020-04-04T19:04:22.0607172Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T19:04:22.0613857Z ##[command]git config gc.auto 0
2020-04-04T19:04:22.0617477Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T19:04:22.0621383Z ##[command]git config --get-all http.proxy
2020-04-04T19:04:22.0629236Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70784/merge:refs/remotes/pull/70784/merge
---
2020-04-04T19:06:56.8514889Z  ---> 3fc1b512c57b
2020-04-04T19:06:56.8515298Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T19:06:56.8519205Z  ---> Using cache
2020-04-04T19:06:56.8520220Z  ---> 5ee4295733f4
2020-04-04T19:06:56.8522023Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T19:06:56.8524017Z  ---> 3d07a0fa42fe
2020-04-04T19:06:56.8590135Z Successfully built 3d07a0fa42fe
2020-04-04T19:06:56.8668819Z Successfully tagged rust-ci:latest
2020-04-04T19:06:56.9273344Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T19:06:56.9273344Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T19:06:56.9289437Z Looks like docker image is the same as before, not uploading
2020-04-04T19:07:03.7745349Z [CI_JOB_NAME=mingw-check]
2020-04-04T19:07:03.8467218Z [CI_JOB_NAME=mingw-check]
2020-04-04T19:07:03.8496545Z == clock drift check ==
2020-04-04T19:07:03.8507807Z   local time: Sat Apr  4 19:07:03 UTC 2020
2020-04-04T19:07:04.1295847Z   network time: Sat, 04 Apr 2020 19:07:04 GMT
2020-04-04T19:07:04.1334719Z Starting sccache server...
2020-04-04T19:07:04.2335716Z configure: processing command line
2020-04-04T19:07:04.2336707Z configure: 
2020-04-04T19:07:04.2337831Z configure: rust.parallel-compiler := True
---
2020-04-04T19:11:09.7988023Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T19:11:09.9370708Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T19:11:10.1612983Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T19:11:10.3068876Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T19:11:10.8538087Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T19:11:13.4551539Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T19:11:14.0060099Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T19:11:16.2457729Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T19:11:16.7309075Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T19:13:17.7651023Z configure: build.locked-deps    := True
2020-04-04T19:13:17.7651356Z configure: llvm.ccache          := sccache
2020-04-04T19:13:17.7652034Z configure: build.cargo-native-static := True
2020-04-04T19:13:17.7652543Z configure: dist.missing-tools   := True
2020-04-04T19:13:17.7653193Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T19:13:17.7653812Z configure: writing `config.toml` in current directory
2020-04-04T19:13:17.7654099Z configure: 
2020-04-04T19:13:17.7656319Z configure: run `python /checkout/x.py --help`
2020-04-04T19:13:17.7656600Z configure: 
---
2020-04-04T19:14:56.2793928Z Hugepagesize:       2048 kB
2020-04-04T19:14:56.2794193Z DirectMap4k:      128960 kB
2020-04-04T19:14:56.2794439Z DirectMap2M:     5113856 kB
2020-04-04T19:14:56.2794685Z DirectMap1G:     4194304 kB
2020-04-04T19:14:56.2795395Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T19:14:57.7283844Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T19:14:57.7283844Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T19:14:57.7291648Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T19:14:57.9806590Z    Compiling unicode-xid v0.2.0
2020-04-04T19:14:58.1228450Z    Compiling syn v1.0.11
2020-04-04T19:14:59.0391830Z    Compiling linked-hash-map v0.5.2
2020-04-04T19:14:59.0862448Z    Compiling lazy_static v1.4.0
2020-04-04T19:14:59.0862448Z    Compiling lazy_static v1.4.0
2020-04-04T19:14:59.2855641Z    Compiling yaml-rust v0.4.3
2020-04-04T19:15:04.1416916Z    Compiling quote v1.0.2
2020-04-04T19:15:20.4210575Z    Compiling thiserror-impl v1.0.5
2020-04-04T19:15:25.8704936Z    Compiling thiserror v1.0.5
2020-04-04T19:15:25.9331199Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T19:15:27.2573240Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T19:15:29.0746041Z Build completed successfully in 0:00:32
2020-04-04T19:15:29.0755380Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T19:15:29.3428114Z     Finished dev [unoptimized] target(s) in 0.20s
2020-04-04T19:15:30.5278597Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T19:17:50.5399035Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T19:17:50.6449501Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T19:17:50.8592450Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T19:17:51.0114380Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T19:17:51.5709690Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T19:17:54.0610463Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T19:17:54.5986636Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T19:17:56.9451228Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T19:17:57.4369361Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T19:22:29.7767592Z Build completed successfully in 0:00:46
2020-04-04T19:22:29.7773552Z + /scripts/validate-toolstate.sh
2020-04-04T19:22:29.7820093Z Cloning into 'rust-toolstate'...
2020-04-04T19:22:30.4319853Z Traceback (most recent call last):
2020-04-04T19:22:30.4320225Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T19:22:30.4320505Z     cur_datetime
2020-04-04T19:22:30.4325439Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T19:22:30.4326642Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T19:22:30.4327335Z KeyError: u'rustc-dev-guide'
2020-04-04T19:22:30.4363252Z   local time: Sat Apr  4 19:22:30 UTC 2020
2020-04-04T19:22:30.4363252Z   local time: Sat Apr  4 19:22:30 UTC 2020
2020-04-04T19:22:30.5960650Z   network time: Sat, 04 Apr 2020 19:22:30 GMT
2020-04-04T19:22:31.9339097Z 
2020-04-04T19:22:31.9339097Z 
2020-04-04T19:22:31.9426456Z ##[error]Bash exited with code '1'.
2020-04-04T19:22:31.9440226Z ##[section]Finishing: Run build
2020-04-04T19:22:31.9495588Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70784/merge to s
2020-04-04T19:22:31.9500595Z Task         : Get sources
2020-04-04T19:22:31.9500986Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T19:22:31.9501331Z Version      : 1.0.0
2020-04-04T19:22:31.9501565Z Author       : Microsoft
2020-04-04T19:22:31.9501565Z Author       : Microsoft
2020-04-04T19:22:31.9501960Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T19:22:31.9502410Z ==============================================================================
2020-04-04T19:22:32.3101453Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T19:22:32.3149125Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70784/merge to s
2020-04-04T19:22:32.3247287Z Cleaning up task key
2020-04-04T19:22:32.3249013Z Start cleaning up orphan processes.
2020-04-04T19:22:32.3451796Z Terminate orphan process: pid (3280) (python)
2020-04-04T19:22:32.3680074Z ##[section]Finishing: Finalize Job
