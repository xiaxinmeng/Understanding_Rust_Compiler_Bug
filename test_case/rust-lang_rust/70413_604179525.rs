plain
2020-03-25T23:44:53.9275302Z ========================== Starting Command Output ===========================
2020-03-25T23:44:53.9278194Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/43a298fa-30ed-4420-a21d-4c6ab9484e7c.sh
2020-03-25T23:44:53.9278507Z 
2020-03-25T23:44:53.9282777Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-25T23:44:53.9303785Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70413/merge to s
2020-03-25T23:44:53.9307043Z Task         : Get sources
2020-03-25T23:44:53.9307382Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T23:44:53.9307692Z Version      : 1.0.0
2020-03-25T23:44:53.9307903Z Author       : Microsoft
---
2020-03-25T23:44:54.9230413Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-25T23:44:54.9236632Z ##[command]git config gc.auto 0
2020-03-25T23:44:54.9242079Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-25T23:44:54.9246290Z ##[command]git config --get-all http.proxy
2020-03-25T23:44:54.9254381Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70413/merge:refs/remotes/pull/70413/merge
---
2020-03-25T23:47:10.4059928Z  ---> 3fc1b512c57b
2020-03-25T23:47:10.4060260Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-25T23:47:10.4062092Z  ---> Using cache
2020-03-25T23:47:10.4062514Z  ---> 5ee4295733f4
2020-03-25T23:47:10.4064168Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-25T23:47:10.4069047Z  ---> 3d07a0fa42fe
2020-03-25T23:47:10.4105994Z Successfully built 3d07a0fa42fe
2020-03-25T23:47:10.4140880Z Successfully tagged rust-ci:latest
2020-03-25T23:47:10.4426942Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-25T23:50:54.7570973Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T23:50:55.2068054Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T23:50:57.1318526Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T23:50:57.7926196Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T23:50:57.8112059Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T23:50:59.4300418Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-25T23:51:18.7683350Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-25T23:51:22.0334555Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-25T23:51:22.3277445Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-25T23:52:43.8108705Z configure: build.locked-deps    := True
2020-03-25T23:52:43.8109026Z configure: llvm.ccache          := sccache
2020-03-25T23:52:43.8109521Z configure: build.cargo-native-static := True
2020-03-25T23:52:43.8110037Z configure: dist.missing-tools   := True
2020-03-25T23:52:43.8110649Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-25T23:52:43.8111229Z configure: writing `config.toml` in current directory
2020-03-25T23:52:43.8111472Z configure: 
2020-03-25T23:52:43.8111905Z configure: run `python /checkout/x.py --help`
2020-03-25T23:52:43.8112137Z configure: 
---
2020-03-25T23:54:09.2252943Z Hugepagesize:       2048 kB
2020-03-25T23:54:09.2253186Z DirectMap4k:      126912 kB
2020-03-25T23:54:09.2253411Z DirectMap2M:     4067328 kB
2020-03-25T23:54:09.2253636Z DirectMap1G:     5242880 kB
2020-03-25T23:54:09.2304915Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-25T23:54:10.5535603Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-25T23:54:10.5535603Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-25T23:54:10.5540469Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-25T23:54:10.7896568Z    Compiling unicode-xid v0.2.0
2020-03-25T23:54:10.9315620Z    Compiling syn v1.0.11
2020-03-25T23:54:11.7521340Z    Compiling linked-hash-map v0.5.2
2020-03-25T23:54:11.7871651Z    Compiling lazy_static v1.4.0
2020-03-25T23:54:11.7871651Z    Compiling lazy_static v1.4.0
2020-03-25T23:54:11.9841109Z    Compiling yaml-rust v0.4.3
2020-03-25T23:54:16.1702869Z    Compiling quote v1.0.2
2020-03-25T23:54:30.1518837Z    Compiling thiserror-impl v1.0.5
2020-03-25T23:54:34.7677264Z    Compiling thiserror v1.0.5
2020-03-25T23:54:34.8272056Z    Compiling yaml-merge-keys v0.4.0
2020-03-25T23:54:35.9996734Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-25T23:54:38.1228939Z Build completed successfully in 0:00:28
2020-03-25T23:54:38.1236609Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-25T23:54:38.3657862Z     Finished dev [unoptimized] target(s) in 0.17s
2020-03-25T23:54:39.4454511Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-25T23:56:41.2191452Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T23:56:41.6919391Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T23:56:43.7171106Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T23:56:44.4153234Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T23:56:44.5212481Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T23:56:46.2333916Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-25T23:57:06.0593987Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-25T23:57:09.0131962Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-25T23:57:09.0316439Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-26T00:00:50.2016647Z Diff in /checkout/src/librustc_mir_build/hair/pattern/_match.rs at line 1719:
2020-03-26T00:00:50.2017494Z              Some(hir_id),
2020-03-26T00:00:50.2017908Z          )
2020-03-26T00:00:50.2018278Z          .into_iter()
2020-03-26T00:00:50.2019884Z -        .map(|c| is_useful_specialized(cx, matrix, v, c, pcx.ty, witness_preference, hir_id, is_under_guard))
2020-03-26T00:00:50.2021247Z +        .map(|c| {
2020-03-26T00:00:50.2021653Z +            is_useful_specialized(
2020-03-26T00:00:50.2022333Z +                matrix,
2020-03-26T00:00:50.2022646Z +                v,
2020-03-26T00:00:50.2022961Z +                c,
2020-03-26T00:00:50.2025991Z +                pcx.ty,
2020-03-26T00:00:50.2025991Z +                pcx.ty,
2020-03-26T00:00:50.2026417Z +                witness_preference,
2020-03-26T00:00:50.2026793Z +                hir_id,
2020-03-26T00:00:50.2027129Z +                is_under_guard,
2020-03-26T00:00:50.2027447Z +            )
2020-03-26T00:00:50.2027742Z +        })
2020-03-26T00:00:50.2028076Z          .find(|result| result.is_useful())
2020-03-26T00:00:50.2028440Z          .unwrap_or(NotUseful)
2020-03-26T00:00:50.2029167Z Diff in /checkout/src/librustc_mir_build/hair/pattern/_match.rs at line 1753:
2020-03-26T00:00:50.2029167Z Diff in /checkout/src/librustc_mir_build/hair/pattern/_match.rs at line 1753:
2020-03-26T00:00:50.2029752Z              split_grouped_constructors(cx.tcx, cx.param_env, pcx, all_ctors, matrix, DUMMY_SP, None)
2020-03-26T00:00:50.2030569Z                  .map(|c| {
2020-03-26T00:00:50.2030569Z                  .map(|c| {
2020-03-26T00:00:50.2031661Z -                    is_useful_specialized(cx, matrix, v, c, pcx.ty, witness_preference, hir_id, is_under_guard)
2020-03-26T00:00:50.2038189Z +                    is_useful_specialized(
2020-03-26T00:00:50.2038993Z +                        matrix,
2020-03-26T00:00:50.2039331Z +                        v,
2020-03-26T00:00:50.2039680Z +                        c,
2020-03-26T00:00:50.2040022Z +                        pcx.ty,
2020-03-26T00:00:50.2040022Z +                        pcx.ty,
2020-03-26T00:00:50.2040394Z +                        witness_preference,
2020-03-26T00:00:50.2093072Z +                        hir_id,
2020-03-26T00:00:50.2093864Z +                        is_under_guard,
2020-03-26T00:00:50.2094266Z +                    )
2020-03-26T00:00:50.2094608Z                  })
2020-03-26T00:00:50.2094972Z                  .find(|result| result.is_useful())
2020-03-26T00:00:50.2095367Z                  .unwrap_or(NotUseful)
2020-03-26T00:00:50.2095824Z Diff in /checkout/src/librustc_mir_build/hair/pattern/_match.rs at line 1760:
2020-03-26T00:00:50.2096228Z          } else {
2020-03-26T00:00:50.2096605Z              let matrix = matrix.specialize_wildcard();
2020-03-26T00:00:50.2096996Z              let v = v.to_tail();
2020-03-26T00:00:50.2097979Z -            let usefulness = is_useful(cx, &matrix, &v, witness_preference, hir_id, is_under_guard, false);
2020-03-26T00:00:50.2098496Z +            let usefulness =
2020-03-26T00:00:50.2098951Z +                is_useful(cx, &matrix, &v, witness_preference, hir_id, is_under_guard, false);
2020-03-26T00:00:50.2099351Z  
2020-03-26T00:00:50.2099939Z              // In this case, there's at least one "free"
2020-03-26T00:00:50.2100398Z              // constructor that is only matched against by
2020-03-26T00:00:50.2103348Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir_build/hair/pattern/_match.rs"` failed.
2020-03-26T00:00:50.2104635Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-26T00:00:50.2105761Z Build completed unsuccessfully in 0:00:41
2020-03-26T00:00:50.2106134Z == clock drift check ==
2020-03-26T00:00:50.2113389Z   local time: Thu Mar 26 00:00:50 UTC 2020
2020-03-26T00:00:50.4990636Z   network time: Thu, 26 Mar 2020 00:00:50 GMT
2020-03-26T00:00:50.4990636Z   network time: Thu, 26 Mar 2020 00:00:50 GMT
2020-03-26T00:00:50.4994479Z == end clock drift check ==
2020-03-26T00:00:51.8601642Z 
2020-03-26T00:00:51.8678372Z ##[error]Bash exited with code '1'.
2020-03-26T00:00:51.8694052Z ##[section]Finishing: Run build
2020-03-26T00:00:51.8748762Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70413/merge to s
2020-03-26T00:00:51.8753879Z Task         : Get sources
2020-03-26T00:00:51.8754256Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T00:00:51.8754589Z Version      : 1.0.0
2020-03-26T00:00:51.8754822Z Author       : Microsoft
2020-03-26T00:00:51.8754822Z Author       : Microsoft
2020-03-26T00:00:51.8755205Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-26T00:00:51.8755627Z ==============================================================================
2020-03-26T00:00:52.2106293Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-26T00:00:52.2161097Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70413/merge to s
2020-03-26T00:00:52.2251353Z Cleaning up task key
2020-03-26T00:00:52.2252673Z Start cleaning up orphan processes.
2020-03-26T00:00:52.2429766Z Terminate orphan process: pid (3530) (python)
2020-03-26T00:00:52.3012376Z ##[section]Finishing: Finalize Job
