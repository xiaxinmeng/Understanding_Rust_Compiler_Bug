plain
2020-03-25T20:24:30.0460855Z ========================== Starting Command Output ===========================
2020-03-25T20:24:30.0463411Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/27c76cbd-294a-4853-95dd-c28d1e2bf221.sh
2020-03-25T20:24:30.0463711Z 
2020-03-25T20:24:30.0466983Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-25T20:24:30.0487229Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-25T20:24:30.0491014Z Task         : Get sources
2020-03-25T20:24:30.0491339Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T20:24:30.0491646Z Version      : 1.0.0
2020-03-25T20:24:30.0491852Z Author       : Microsoft
---
2020-03-25T20:24:31.2742137Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-25T20:24:31.2753187Z ##[command]git config gc.auto 0
2020-03-25T20:24:31.2759182Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-25T20:24:31.2779879Z ##[command]git config --get-all http.proxy
2020-03-25T20:24:31.2805391Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69916/merge:refs/remotes/pull/69916/merge
---
2020-03-25T20:27:56.1636689Z  ---> 3fc1b512c57b
2020-03-25T20:27:56.1637180Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-25T20:27:56.1637959Z  ---> Using cache
2020-03-25T20:27:56.1638489Z  ---> 5ee4295733f4
2020-03-25T20:27:56.1640505Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-25T20:27:56.1642245Z  ---> 3d07a0fa42fe
2020-03-25T20:27:56.1662096Z Successfully built 3d07a0fa42fe
2020-03-25T20:27:56.1703816Z Successfully tagged rust-ci:latest
2020-03-25T20:27:56.1999366Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-25T20:31:38.2362982Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T20:31:38.7142561Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T20:31:41.4085304Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T20:31:41.4178780Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T20:31:41.4484074Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T20:31:43.1308932Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-25T20:32:03.2724351Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-25T20:32:06.9183258Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-25T20:32:06.9184532Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-25T20:33:29.2532186Z configure: build.locked-deps    := True
2020-03-25T20:33:29.2532509Z configure: llvm.ccache          := sccache
2020-03-25T20:33:29.2532997Z configure: build.cargo-native-static := True
2020-03-25T20:33:29.2533475Z configure: dist.missing-tools   := True
2020-03-25T20:33:29.2534085Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-25T20:33:29.2534649Z configure: writing `config.toml` in current directory
2020-03-25T20:33:29.2534909Z configure: 
2020-03-25T20:33:29.2535315Z configure: run `python /checkout/x.py --help`
2020-03-25T20:33:29.2535549Z configure: 
---
2020-03-25T20:34:53.3773451Z Hugepagesize:       2048 kB
2020-03-25T20:34:53.3773694Z DirectMap4k:      143296 kB
2020-03-25T20:34:53.3773922Z DirectMap2M:     5099520 kB
2020-03-25T20:34:53.3774151Z DirectMap1G:     4194304 kB
2020-03-25T20:34:53.3796975Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-25T20:34:54.7345204Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-25T20:34:54.7345204Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-25T20:34:54.7355046Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-25T20:34:54.9686318Z    Compiling unicode-xid v0.2.0
2020-03-25T20:34:55.0959859Z    Compiling syn v1.0.11
2020-03-25T20:34:55.9383367Z    Compiling linked-hash-map v0.5.2
2020-03-25T20:34:55.9566966Z    Compiling lazy_static v1.4.0
2020-03-25T20:34:55.9566966Z    Compiling lazy_static v1.4.0
2020-03-25T20:34:56.1662189Z    Compiling yaml-rust v0.4.3
2020-03-25T20:35:00.3639193Z    Compiling quote v1.0.2
2020-03-25T20:35:14.9867454Z    Compiling thiserror-impl v1.0.5
2020-03-25T20:35:19.1896476Z    Compiling thiserror v1.0.5
2020-03-25T20:35:19.6905161Z    Compiling yaml-merge-keys v0.4.0
2020-03-25T20:35:20.8670503Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-25T20:35:22.4552593Z Build completed successfully in 0:00:29
2020-03-25T20:35:22.4564233Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-25T20:35:22.7062626Z     Finished dev [unoptimized] target(s) in 0.18s
2020-03-25T20:35:23.8127487Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-25T20:37:28.0439479Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T20:37:28.5484514Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T20:37:30.5568565Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T20:37:31.2571992Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T20:37:31.3694303Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T20:37:33.0961954Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-25T20:37:53.2413707Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-25T20:37:56.1745261Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-25T20:37:56.1884816Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-25T20:41:39.4154251Z Diff in /checkout/src/librustc_mir/util/pretty.rs at line 80:
2020-03-25T20:41:39.4154596Z          return;
2020-03-25T20:41:39.4154764Z      }
2020-03-25T20:41:39.4159550Z  
2020-03-25T20:41:39.4160684Z -    dump_matched_mir_node(
2020-03-25T20:41:39.4161262Z -        tcx,
2020-03-25T20:41:39.4161752Z -        pass_num,
2020-03-25T20:41:39.4162240Z -        pass_name,
2020-03-25T20:41:39.4163266Z -        source,
2020-03-25T20:41:39.4163725Z -        body,
2020-03-25T20:41:39.4164215Z -        extra_data,
2020-03-25T20:41:39.4164680Z -    );
2020-03-25T20:41:39.4164680Z -    );
2020-03-25T20:41:39.4165116Z +    dump_matched_mir_node(tcx, pass_num, pass_name, disambiguator, source, body, extra_data);
2020-03-25T20:41:39.4165776Z  
2020-03-25T20:41:39.4165776Z  
2020-03-25T20:41:39.4166472Z  pub fn dump_enabled<'tcx>(tcx: TyCtxt<'tcx>, pass_name: &str, source: MirSource<'tcx>) -> bool {
2020-03-25T20:41:39.4167791Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/util/pretty.rs"` failed.
2020-03-25T20:41:39.4168941Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-25T20:41:39.4186022Z Build completed unsuccessfully in 0:00:40
2020-03-25T20:41:39.4237682Z == clock drift check ==
2020-03-25T20:41:39.4253470Z   local time: Wed Mar 25 20:41:39 UTC 2020
2020-03-25T20:41:39.7131948Z   network time: Wed, 25 Mar 2020 20:41:39 GMT
2020-03-25T20:41:39.7131948Z   network time: Wed, 25 Mar 2020 20:41:39 GMT
2020-03-25T20:41:39.7140186Z == end clock drift check ==
2020-03-25T20:41:41.2734656Z 
2020-03-25T20:41:41.2805749Z ##[error]Bash exited with code '1'.
2020-03-25T20:41:41.2819456Z ##[section]Finishing: Run build
2020-03-25T20:41:41.2868000Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-25T20:41:41.2873010Z Task         : Get sources
2020-03-25T20:41:41.2873379Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T20:41:41.2873728Z Version      : 1.0.0
2020-03-25T20:41:41.2873969Z Author       : Microsoft
2020-03-25T20:41:41.2873969Z Author       : Microsoft
2020-03-25T20:41:41.2874354Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-25T20:41:41.2874801Z ==============================================================================
2020-03-25T20:41:41.6154662Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-25T20:41:41.6199019Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-25T20:41:41.6288710Z Cleaning up task key
2020-03-25T20:41:41.6290167Z Start cleaning up orphan processes.
2020-03-25T20:41:41.6466476Z Terminate orphan process: pid (3506) (python)
2020-03-25T20:41:41.6622277Z ##[section]Finishing: Finalize Job
