plain
2020-03-30T17:38:52.2041223Z ========================== Starting Command Output ===========================
2020-03-30T17:38:52.2044173Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d3b0b8fa-6790-47f8-8001-fa79725ed9db.sh
2020-03-30T17:38:52.2044449Z 
2020-03-30T17:38:52.2049924Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T17:38:52.2069684Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-30T17:38:52.2072970Z Task         : Get sources
2020-03-30T17:38:52.2073283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T17:38:52.2073586Z Version      : 1.0.0
2020-03-30T17:38:52.2073791Z Author       : Microsoft
---
2020-03-30T17:38:53.4768512Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T17:38:53.4780791Z ##[command]git config gc.auto 0
2020-03-30T17:38:53.4784807Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T17:38:53.4788374Z ##[command]git config --get-all http.proxy
2020-03-30T17:38:53.4795882Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70458/merge:refs/remotes/pull/70458/merge
---
2020-03-30T17:42:15.1937017Z  ---> 3fc1b512c57b
2020-03-30T17:42:15.1937263Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-30T17:42:15.1937648Z  ---> Using cache
2020-03-30T17:42:15.1937983Z  ---> 5ee4295733f4
2020-03-30T17:42:15.1939376Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-30T17:42:15.1981232Z  ---> 3d07a0fa42fe
2020-03-30T17:42:15.1981453Z Successfully built 3d07a0fa42fe
2020-03-30T17:42:15.2063735Z Successfully tagged rust-ci:latest
2020-03-30T17:42:15.3068229Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-30T17:45:58.9936989Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T17:45:59.5660162Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T17:45:59.5666410Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T17:45:59.5667080Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T17:45:59.9323244Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T17:46:02.1681547Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T17:46:02.6283113Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T17:46:04.6755498Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T17:46:05.0992075Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T17:47:52.4818527Z configure: build.locked-deps    := True
2020-03-30T17:47:52.4818823Z configure: llvm.ccache          := sccache
2020-03-30T17:47:52.4819341Z configure: build.cargo-native-static := True
2020-03-30T17:47:52.4819839Z configure: dist.missing-tools   := True
2020-03-30T17:47:52.4820438Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-30T17:47:52.4821009Z configure: writing `config.toml` in current directory
2020-03-30T17:47:52.4821248Z configure: 
2020-03-30T17:47:52.4821658Z configure: run `python /checkout/x.py --help`
2020-03-30T17:47:52.4821884Z configure: 
---
2020-03-30T17:49:17.6729357Z Hugepagesize:       2048 kB
2020-03-30T17:49:17.6729773Z DirectMap4k:      133056 kB
2020-03-30T17:49:17.6729994Z DirectMap2M:     4061184 kB
2020-03-30T17:49:17.6730212Z DirectMap1G:     5242880 kB
2020-03-30T17:49:17.6803576Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-30T17:49:19.0488748Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-30T17:49:19.0488748Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-30T17:49:19.0534457Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-30T17:49:19.2912713Z    Compiling unicode-xid v0.2.0
2020-03-30T17:49:19.4230146Z    Compiling syn v1.0.11
2020-03-30T17:49:20.2566687Z    Compiling linked-hash-map v0.5.2
2020-03-30T17:49:20.3022766Z    Compiling lazy_static v1.4.0
2020-03-30T17:49:20.3022766Z    Compiling lazy_static v1.4.0
2020-03-30T17:49:20.4872443Z    Compiling yaml-rust v0.4.3
2020-03-30T17:49:24.8188342Z    Compiling quote v1.0.2
2020-03-30T17:49:39.2172703Z    Compiling thiserror-impl v1.0.5
2020-03-30T17:49:43.9253359Z    Compiling thiserror v1.0.5
2020-03-30T17:49:43.9901298Z    Compiling yaml-merge-keys v0.4.0
2020-03-30T17:49:45.1405372Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-30T17:49:48.7051722Z Build completed successfully in 0:00:30
2020-03-30T17:49:48.7058711Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-30T17:49:48.9451617Z     Finished dev [unoptimized] target(s) in 0.17s
2020-03-30T17:49:50.0441919Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-30T17:51:52.7017751Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T17:51:52.9067968Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T17:51:53.0973898Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T17:51:53.1097938Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T17:51:53.7165824Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T17:51:55.9820085Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T17:51:56.4710832Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T17:51:58.5189308Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T17:51:58.9544287Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T17:55:57.3204913Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-30T17:55:57.3205668Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-30T17:55:57.3208291Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-30T17:55:57.3222043Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-30T17:56:01.1366946Z Diff in /checkout/src/librustc_codegen_llvm/back/lto.rs at line 16:
2020-03-30T17:56:01.1367381Z  use rustc_middle::bug;
2020-03-30T17:56:01.1367654Z  use rustc_middle::dep_graph::WorkProduct;
2020-03-30T17:56:01.1367992Z  use rustc_middle::middle::exported_symbols::SymbolExportLevel;
2020-03-30T17:56:01.1369018Z -use rustc_session::cgu_reuse_tracker::CguReuse;
2020-03-30T17:56:01.1369305Z  use rustc_serialize::leb128;
2020-03-30T17:56:01.1369584Z +use rustc_session::cgu_reuse_tracker::CguReuse;
2020-03-30T17:56:01.1369900Z  use rustc_session::config::{self, Lto};
2020-03-30T17:56:01.1370099Z  
2020-03-30T17:56:01.1370280Z  use std::ffi::{CStr, CString};
2020-03-30T17:56:01.1382272Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_codegen_llvm/back/lto.rs"` failed.
2020-03-30T17:56:01.1383286Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-30T17:56:01.1398696Z Build completed unsuccessfully in 0:00:39
2020-03-30T17:56:01.1449980Z == clock drift check ==
2020-03-30T17:56:01.1469360Z   local time: Mon Mar 30 17:56:01 UTC 2020
2020-03-30T17:56:01.1712051Z   network time: Diff in /checkout/src/librustc_codegen_llvm/back/write.rs at line 19:
2020-03-30T17:56:01.1712051Z   network time: Diff in /checkout/src/librustc_codegen_llvm/back/write.rs at line 19:
2020-03-30T17:56:01.1712499Z  use rustc_data_structures::small_c_str::SmallCStr;
2020-03-30T17:56:01.1712812Z  use rustc_errors::{FatalError, Handler};
2020-03-30T17:56:01.1713200Z  use rustc_fs_util::{link_or_copy, path_to_c_string};
2020-03-30T17:56:01.1714213Z -use rustc_serialize::leb128;
2020-03-30T17:56:01.1714718Z  use rustc_middle::bug;
2020-03-30T17:56:01.1714965Z  use rustc_middle::ty::TyCtxt;
2020-03-30T17:56:01.1715313Z Diff in /checkout/src/librustc_codegen_llvm/back/write.rs at line 26:
2020-03-30T17:56:01.1715313Z Diff in /checkout/src/librustc_codegen_llvm/back/write.rs at line 26:
2020-03-30T17:56:01.1715636Z +use rustc_serialize::leb128;
2020-03-30T17:56:01.1716006Z  use rustc_session::config::{self, Lto, OutputType, Passes, Sanitizer, SwitchWithOptPath};
2020-03-30T17:56:01.1716527Z  
2020-03-30T17:56:01.1716782Z Diff in /checkout/src/librustc_codegen_llvm/back/write.rs at line 948:
2020-03-30T17:56:01.1717077Z              .filter_map(|val| {
2020-03-30T17:56:01.1717381Z                  // Exclude some symbols that we know are not Rust symbols.
2020-03-30T17:56:01.1717381Z                  // Exclude some symbols that we know are not Rust symbols.
2020-03-30T17:56:01.1717754Z                  let name = llvm::get_value_name(val);
2020-03-30T17:56:01.1718259Z -                if ignored(name) {
2020-03-30T17:56:01.1719052Z -                } else {
2020-03-30T17:56:01.1719467Z -                    Some((val, name))
2020-03-30T17:56:01.1719847Z -                }
2020-03-30T17:56:01.1719847Z -                }
2020-03-30T17:56:01.1720134Z +                if ignored(name) { None } else { Some((val, name)) }
2020-03-30T17:56:01.1720599Z              })
2020-03-30T17:56:01.1726318Z              .map(move |(val, name)| {
2020-03-30T17:56:01.1726835Z                  let mut imp_name = prefix.as_bytes().to_vec();
2020-03-30T17:56:01.1727473Z == end clock drift check ==
2020-03-30T17:56:02.7098203Z 
2020-03-30T17:56:02.7098203Z 
2020-03-30T17:56:02.7164917Z ##[error]Bash exited with code '1'.
2020-03-30T17:56:02.7178947Z ##[section]Finishing: Run build
2020-03-30T17:56:02.7224784Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-30T17:56:02.7229544Z Task         : Get sources
2020-03-30T17:56:02.7229892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T17:56:02.7230228Z Version      : 1.0.0
2020-03-30T17:56:02.7230450Z Author       : Microsoft
2020-03-30T17:56:02.7230450Z Author       : Microsoft
2020-03-30T17:56:02.7230801Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T17:56:02.7231235Z ==============================================================================
2020-03-30T17:56:03.0500050Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T17:56:03.0544188Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-30T17:56:03.0636067Z Cleaning up task key
2020-03-30T17:56:03.0637325Z Start cleaning up orphan processes.
2020-03-30T17:56:03.1052884Z Terminate orphan process: pid (4433) (python)
2020-03-30T17:56:03.1102519Z ##[section]Finishing: Finalize Job
