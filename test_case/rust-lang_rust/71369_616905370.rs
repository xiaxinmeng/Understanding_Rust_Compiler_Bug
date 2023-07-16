plain
2020-04-21T00:14:57.1521076Z ========================== Starting Command Output ===========================
2020-04-21T00:14:57.1523458Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/28aef0f0-7345-4fa5-bc96-08ee829a8ae0.sh
2020-04-21T00:14:57.1523692Z 
2020-04-21T00:14:57.1527122Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T00:14:57.1545437Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71369/merge to s
2020-04-21T00:14:57.1548622Z Task         : Get sources
2020-04-21T00:14:57.1548915Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T00:14:57.1549183Z Version      : 1.0.0
2020-04-21T00:14:57.1549370Z Author       : Microsoft
---
2020-04-21T00:14:58.1487848Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T00:14:58.1494403Z ##[command]git config gc.auto 0
2020-04-21T00:14:58.1513435Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T00:14:58.1517928Z ##[command]git config --get-all http.proxy
2020-04-21T00:14:58.1533197Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71369/merge:refs/remotes/pull/71369/merge
---
2020-04-21T00:18:41.7718544Z  ---> 78ad2f4d4aca
2020-04-21T00:18:41.7718969Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-21T00:18:41.7725485Z  ---> Using cache
2020-04-21T00:18:41.7726141Z  ---> 4d2dc61c4d00
2020-04-21T00:18:41.7727783Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-21T00:18:41.7735486Z  ---> 776b6266a8b7
2020-04-21T00:18:41.7769052Z Successfully built 776b6266a8b7
2020-04-21T00:18:41.7790381Z Successfully tagged rust-ci:latest
2020-04-21T00:18:41.8149208Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T00:18:41.8149208Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T00:18:41.8165317Z Looks like docker image is the same as before, not uploading
2020-04-21T00:18:42.6220746Z [CI_JOB_NAME=mingw-check]
2020-04-21T00:18:42.6539835Z [CI_JOB_NAME=mingw-check]
2020-04-21T00:18:42.6565600Z == clock drift check ==
2020-04-21T00:18:42.6572523Z   local time: Tue Apr 21 00:18:42 UTC 2020
2020-04-21T00:18:42.7772441Z   network time: Tue, 21 Apr 2020 00:18:42 GMT
2020-04-21T00:18:42.7796539Z Starting sccache server...
2020-04-21T00:18:42.8736644Z configure: processing command line
2020-04-21T00:18:42.8736859Z configure: 
2020-04-21T00:18:42.8737621Z configure: rust.parallel-compiler := True
---
2020-04-21T00:22:23.9409193Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T00:22:24.2258648Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T00:22:24.3207850Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T00:22:24.4330808Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T00:22:24.9076356Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T00:22:26.9638264Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T00:22:27.3976062Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T00:22:29.2556642Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T00:22:29.6559489Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T00:24:08.8508795Z configure: llvm.assertions      := True
2020-04-21T00:24:08.8509036Z configure: build.submodules     := False
2020-04-21T00:24:08.8509296Z configure: rust.channel         := nightly
2020-04-21T00:24:08.8509732Z configure: build.locked-deps    := True
2020-04-21T00:24:08.8510236Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-21T00:24:08.8510722Z configure: writing `config.toml` in current directory
2020-04-21T00:24:08.8510923Z configure: 
2020-04-21T00:24:08.8511274Z configure: run `python /checkout/x.py --help`
2020-04-21T00:24:08.8511483Z configure: 
---
2020-04-21T00:25:39.3415974Z Hugepagesize:       2048 kB
2020-04-21T00:25:39.3416174Z DirectMap4k:      141248 kB
2020-04-21T00:25:39.3416390Z DirectMap2M:     4052992 kB
2020-04-21T00:25:39.3416590Z DirectMap1G:     5242880 kB
2020-04-21T00:25:39.3445092Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-21T00:25:40.6538823Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T00:25:40.6538823Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T00:25:40.6544655Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-21T00:25:40.8722380Z    Compiling unicode-xid v0.2.0
2020-04-21T00:25:40.9914834Z    Compiling syn v1.0.11
2020-04-21T00:25:41.7841829Z    Compiling linked-hash-map v0.5.2
2020-04-21T00:25:41.8371574Z    Compiling lazy_static v1.4.0
2020-04-21T00:25:41.8371574Z    Compiling lazy_static v1.4.0
2020-04-21T00:25:41.9946849Z    Compiling yaml-rust v0.4.3
2020-04-21T00:25:46.1240854Z    Compiling quote v1.0.2
2020-04-21T00:25:59.6457760Z    Compiling thiserror-impl v1.0.5
2020-04-21T00:26:04.0206123Z    Compiling thiserror v1.0.5
2020-04-21T00:26:04.0825409Z    Compiling yaml-merge-keys v0.4.0
2020-04-21T00:26:05.1518669Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-21T00:26:06.7166717Z Build completed successfully in 0:00:27
2020-04-21T00:26:06.7261519Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-21T00:26:06.9870396Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-21T00:26:08.0421147Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-21T00:28:06.6037333Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T00:28:06.7653795Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T00:28:06.9683123Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T00:28:06.9837236Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T00:28:07.5318664Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T00:28:09.5714408Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T00:28:10.0107307Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T00:28:11.8653627Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T00:28:12.2623768Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T00:31:58.5080399Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-21T00:31:58.5085000Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-21T00:31:59.7573641Z Diff in /checkout/src/librustc_data_structures/profiling.rs at line 99:
2020-04-21T00:31:59.7574349Z  
2020-04-21T00:31:59.7574755Z  /// MmapSerializatioSink is faster on macOS and Linux
2020-04-21T00:31:59.7575137Z  /// but FileSerializationSink is faster on Windows
2020-04-21T00:31:59.7576139Z -#[cfg(all(not(windows),not(target_arch="wasm32")))]
2020-04-21T00:31:59.7576530Z +#[cfg(all(not(windows), not(target_arch = "wasm32")))]
2020-04-21T00:31:59.7595877Z  type SerializationSink = measureme::MmapSerializationSink;
2020-04-21T00:31:59.7606105Z -#[cfg(all(windows,not(target_arch="wasm32")))]
2020-04-21T00:31:59.7606898Z +#[cfg(all(windows, not(target_arch = "wasm32")))]
2020-04-21T00:31:59.7608007Z  type SerializationSink = measureme::FileSerializationSink;
2020-04-21T00:31:59.7610297Z -#[cfg(target_arch="wasm32")]
2020-04-21T00:31:59.7611738Z +#[cfg(target_arch = "wasm32")]
2020-04-21T00:31:59.7618261Z  type SerializationSink = measureme::ByteVecSink;
2020-04-21T00:31:59.7618589Z  
2020-04-21T00:31:59.7618860Z  type Profiler = measureme::Profiler<SerializationSink>;
2020-04-21T00:31:59.7619238Z Diff in /checkout/src/librustc_data_structures/profiling.rs at line 604:
2020-04-21T00:31:59.7619713Z  
2020-04-21T00:31:59.7619929Z  // Memory reporting
2020-04-21T00:31:59.7619929Z  // Memory reporting
2020-04-21T00:31:59.7620509Z -#[cfg(all(unix,not(target_arch="wasm32")))]
2020-04-21T00:31:59.7620823Z +#[cfg(all(unix, not(target_arch = "wasm32")))]
2020-04-21T00:31:59.7621533Z      let field = 1;
2020-04-21T00:31:59.7621533Z      let field = 1;
2020-04-21T00:31:59.7621814Z      let contents = fs::read("/proc/self/statm").ok()?;
2020-04-21T00:31:59.7622188Z Diff in /checkout/src/librustc_data_structures/profiling.rs at line 614:
2020-04-21T00:31:59.7622492Z      Some(npages * 4096)
2020-04-21T00:31:59.7622893Z  
2020-04-21T00:31:59.7622893Z  
2020-04-21T00:31:59.7623805Z -#[cfg(all(windows,not(target_arch="wasm32")))]
2020-04-21T00:31:59.7624152Z +#[cfg(all(windows, not(target_arch = "wasm32")))]
2020-04-21T00:31:59.7624877Z      use std::mem::{self, MaybeUninit};
2020-04-21T00:31:59.7624877Z      use std::mem::{self, MaybeUninit};
2020-04-21T00:31:59.7629226Z      use winapi::shared::minwindef::DWORD;
2020-04-21T00:31:59.7629859Z Diff in /checkout/src/librustc_data_structures/profiling.rs at line 633:
2020-04-21T00:31:59.7630367Z  }
2020-04-21T00:31:59.7630698Z  
2020-04-21T00:31:59.7630698Z  
2020-04-21T00:31:59.7631236Z -#[cfg(target_arch="wasm32")]
2020-04-21T00:31:59.7631711Z +#[cfg(target_arch = "wasm32")]
2020-04-21T00:31:59.7632483Z      None
2020-04-21T00:31:59.7632705Z  }
2020-04-21T00:31:59.7632705Z  }
2020-04-21T00:31:59.7633629Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_data_structures/profiling.rs"` failed.
2020-04-21T00:31:59.7636048Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-21T00:31:59.7641744Z Build completed unsuccessfully in 0:00:38
2020-04-21T00:31:59.7766434Z == clock drift check ==
2020-04-21T00:31:59.7766434Z == clock drift check ==
2020-04-21T00:31:59.7778681Z   local time: Tue Apr 21 00:31:59 UTC 2020
2020-04-21T00:32:00.0968275Z   network time: Tue, 21 Apr 2020 00:32:00 GMT
2020-04-21T00:32:01.6820380Z 
2020-04-21T00:32:01.6820380Z 
2020-04-21T00:32:01.6893358Z ##[error]Bash exited with code '1'.
2020-04-21T00:32:01.6926254Z ##[section]Finishing: Run build
2020-04-21T00:32:01.6971881Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71369/merge to s
2020-04-21T00:32:01.6976416Z Task         : Get sources
2020-04-21T00:32:01.6976714Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T00:32:01.6976995Z Version      : 1.0.0
2020-04-21T00:32:01.6977190Z Author       : Microsoft
2020-04-21T00:32:01.6977190Z Author       : Microsoft
2020-04-21T00:32:01.6977501Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T00:32:01.6977875Z ==============================================================================
2020-04-21T00:32:02.0180311Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T00:32:02.0222653Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71369/merge to s
2020-04-21T00:32:02.0304300Z Cleaning up task key
2020-04-21T00:32:02.0305468Z Start cleaning up orphan processes.
2020-04-21T00:32:02.0469323Z Terminate orphan process: pid (4051) (python)
2020-04-21T00:32:02.0771939Z ##[section]Finishing: Finalize Job
