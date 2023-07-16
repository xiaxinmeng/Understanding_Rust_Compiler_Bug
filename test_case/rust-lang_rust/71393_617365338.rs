plain
2020-04-21T17:54:10.6623084Z ========================== Starting Command Output ===========================
2020-04-21T17:54:10.6627953Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2cefe627-e8a5-4bdb-86ad-d5dad94d85e8.sh
2020-04-21T17:54:10.6628377Z 
2020-04-21T17:54:10.6632810Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T17:54:10.6651456Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71393/merge to s
2020-04-21T17:54:10.6654620Z Task         : Get sources
2020-04-21T17:54:10.6654891Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T17:54:10.6655155Z Version      : 1.0.0
2020-04-21T17:54:10.6655337Z Author       : Microsoft
---
2020-04-21T17:54:11.6592684Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T17:54:11.6598824Z ##[command]git config gc.auto 0
2020-04-21T17:54:11.6602869Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T17:54:11.6606635Z ##[command]git config --get-all http.proxy
2020-04-21T17:54:11.6613460Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71393/merge:refs/remotes/pull/71393/merge
---
2020-04-21T17:56:46.1316503Z  ---> 78ad2f4d4aca
2020-04-21T17:56:46.1317126Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-21T17:56:46.1321458Z  ---> Using cache
2020-04-21T17:56:46.1322200Z  ---> 4d2dc61c4d00
2020-04-21T17:56:46.1334233Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-21T17:56:46.1340777Z  ---> 776b6266a8b7
2020-04-21T17:56:46.1440201Z Successfully built 776b6266a8b7
2020-04-21T17:56:46.1541363Z Successfully tagged rust-ci:latest
2020-04-21T17:56:47.2850897Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T17:56:47.2850897Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T17:56:47.2870200Z Looks like docker image is the same as before, not uploading
2020-04-21T17:56:54.2983183Z [CI_JOB_NAME=mingw-check]
2020-04-21T17:56:54.3281751Z [CI_JOB_NAME=mingw-check]
2020-04-21T17:56:54.3311148Z == clock drift check ==
2020-04-21T17:56:54.3318448Z   local time: Tue Apr 21 17:56:54 UTC 2020
2020-04-21T17:56:54.3959995Z   network time: Tue, 21 Apr 2020 17:56:54 GMT
2020-04-21T17:56:54.3986790Z Starting sccache server...
2020-04-21T17:56:54.5262273Z configure: processing command line
2020-04-21T17:56:54.5262670Z configure: 
2020-04-21T17:56:54.5263637Z configure: rust.parallel-compiler := True
---
2020-04-21T18:01:09.6988445Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-21T18:01:14.8958155Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-21T18:01:16.2398644Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T18:01:16.2685764Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T18:01:16.4751067Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T18:01:17.3953646Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T18:01:17.4126509Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-21T18:01:19.0398705Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T18:01:19.5794829Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-21T18:03:30.6187701Z configure: rust.verify-llvm-ir  := True
2020-04-21T18:03:30.6188004Z configure: llvm.assertions      := True
2020-04-21T18:03:30.6188522Z configure: build.cargo-native-static := True
2020-04-21T18:03:30.6189070Z configure: rust.debug-assertions := True
2020-04-21T18:03:30.6189713Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-21T18:03:30.6190310Z configure: writing `config.toml` in current directory
2020-04-21T18:03:30.6190569Z configure: 
2020-04-21T18:03:30.6191025Z configure: run `python /checkout/x.py --help`
2020-04-21T18:03:30.6191282Z configure: 
---
2020-04-21T18:05:19.5248181Z Hugepagesize:       2048 kB
2020-04-21T18:05:19.5248390Z DirectMap4k:      147392 kB
2020-04-21T18:05:19.5248615Z DirectMap2M:     4046848 kB
2020-04-21T18:05:19.5248821Z DirectMap1G:     5242880 kB
2020-04-21T18:05:19.5272975Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-21T18:05:21.0802575Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T18:05:21.0802575Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T18:05:21.0824485Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-21T18:05:21.3607707Z    Compiling unicode-xid v0.2.0
2020-04-21T18:05:21.4969013Z    Compiling syn v1.0.11
2020-04-21T18:05:22.4780609Z    Compiling linked-hash-map v0.5.2
2020-04-21T18:05:22.5148771Z    Compiling lazy_static v1.4.0
2020-04-21T18:05:22.5148771Z    Compiling lazy_static v1.4.0
2020-04-21T18:05:22.7405839Z    Compiling yaml-rust v0.4.3
2020-04-21T18:05:27.6851110Z    Compiling quote v1.0.2
2020-04-21T18:05:44.4395734Z    Compiling thiserror-impl v1.0.5
2020-04-21T18:05:49.8340766Z    Compiling thiserror v1.0.5
2020-04-21T18:05:49.9002552Z    Compiling yaml-merge-keys v0.4.0
2020-04-21T18:05:51.2462270Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-21T18:05:53.0402548Z Build completed successfully in 0:00:33
2020-04-21T18:05:53.0501745Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-21T18:05:53.3619141Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-21T18:05:54.5980370Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-21T18:08:18.6019897Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T18:08:18.9145154Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T18:08:19.1032933Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T18:08:19.1288332Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T18:08:19.8135710Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T18:08:22.3865253Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T18:08:22.9239440Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T18:08:25.3646670Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T18:08:25.8894044Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T18:13:03.3405948Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-21T18:13:03.3408520Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-21T18:13:03.3414610Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-21T18:13:03.3419424Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-21T18:13:06.8794734Z Diff in /checkout/src/librustc_typeck/check/cast.rs at line 613:
2020-04-21T18:13:06.8795902Z              (Int(CEnum), Int(_)) => Ok(CastKind::EnumCast),
2020-04-21T18:13:06.8798926Z              (Int(Char) | Int(Bool), Int(_)) => Ok(CastKind::PrimIntCast),
2020-04-21T18:13:06.8799546Z  
2020-04-21T18:13:06.8800938Z -            (Float, Int(_)) if std::env::var_os("RUSTC_BOOTSTRAP").is_none() => Err(CastError::IllegalCast),
2020-04-21T18:13:06.8801738Z +            (Float, Int(_)) if std::env::var_os("RUSTC_BOOTSTRAP").is_none() => {
2020-04-21T18:13:06.8802288Z +                Err(CastError::IllegalCast)
2020-04-21T18:13:06.8802676Z +            }
2020-04-21T18:13:06.8803165Z              (Int(_) | Float, Int(_) | Float) => Ok(CastKind::NumericCast),
2020-04-21T18:13:06.8803932Z      }
2020-04-21T18:13:06.8803932Z      }
2020-04-21T18:13:06.8805213Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_typeck/check/cast.rs"` failed.
2020-04-21T18:13:06.8806524Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-21T18:13:06.8811430Z Build completed unsuccessfully in 0:00:45
2020-04-21T18:13:06.8926281Z == clock drift check ==
2020-04-21T18:13:06.8926281Z == clock drift check ==
2020-04-21T18:13:06.8942495Z   local time: Tue Apr 21 18:13:06 UTC 2020
2020-04-21T18:13:07.0944184Z   network time: Tue, 21 Apr 2020 18:13:07 GMT
2020-04-21T18:13:08.3900044Z 
2020-04-21T18:13:08.3900044Z 
2020-04-21T18:13:08.3980735Z ##[error]Bash exited with code '1'.
2020-04-21T18:13:08.3997313Z ##[section]Finishing: Run build
2020-04-21T18:13:08.4046939Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71393/merge to s
2020-04-21T18:13:08.4051902Z Task         : Get sources
2020-04-21T18:13:08.4052225Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T18:13:08.4052540Z Version      : 1.0.0
2020-04-21T18:13:08.4052755Z Author       : Microsoft
2020-04-21T18:13:08.4052755Z Author       : Microsoft
2020-04-21T18:13:08.4053090Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T18:13:08.4053485Z ==============================================================================
2020-04-21T18:13:08.7881351Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T18:13:08.7937456Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71393/merge to s
2020-04-21T18:13:08.8049263Z Cleaning up task key
2020-04-21T18:13:08.8050668Z Start cleaning up orphan processes.
2020-04-21T18:13:08.8276761Z Terminate orphan process: pid (3792) (python)
2020-04-21T18:13:08.8452950Z ##[section]Finishing: Finalize Job
