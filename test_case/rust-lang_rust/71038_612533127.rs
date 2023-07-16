plain
2020-04-11T21:59:45.0073491Z ========================== Starting Command Output ===========================
2020-04-11T21:59:45.0075723Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f3f142f7-6748-4741-86c0-fec9af16cfd5.sh
2020-04-11T21:59:45.0075987Z 
2020-04-11T21:59:45.0079519Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T21:59:45.0097303Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71038/merge to s
2020-04-11T21:59:45.0100389Z Task         : Get sources
2020-04-11T21:59:45.0100669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T21:59:45.0100946Z Version      : 1.0.0
2020-04-11T21:59:45.0101145Z Author       : Microsoft
---
2020-04-11T21:59:46.0046186Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T21:59:46.0053714Z ##[command]git config gc.auto 0
2020-04-11T21:59:46.0058957Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T21:59:46.0064235Z ##[command]git config --get-all http.proxy
2020-04-11T21:59:46.0072866Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71038/merge:refs/remotes/pull/71038/merge
---
2020-04-11T22:02:00.2552171Z  ---> 78ad2f4d4aca
2020-04-11T22:02:00.2552369Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-11T22:02:00.2557471Z  ---> Using cache
2020-04-11T22:02:00.2557840Z  ---> 4d2dc61c4d00
2020-04-11T22:02:00.2559089Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-11T22:02:00.2565624Z  ---> 776b6266a8b7
2020-04-11T22:02:00.2596833Z Successfully built 776b6266a8b7
2020-04-11T22:02:00.2631427Z Successfully tagged rust-ci:latest
2020-04-11T22:02:00.2877461Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T22:02:00.2877461Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T22:02:00.2890577Z Looks like docker image is the same as before, not uploading
2020-04-11T22:02:06.9366475Z [CI_JOB_NAME=mingw-check]
2020-04-11T22:02:06.9614938Z [CI_JOB_NAME=mingw-check]
2020-04-11T22:02:06.9640766Z == clock drift check ==
2020-04-11T22:02:06.9650478Z   local time: Sat Apr 11 22:02:06 UTC 2020
2020-04-11T22:02:07.1510304Z   network time: Sat, 11 Apr 2020 22:02:07 GMT
2020-04-11T22:02:07.1551693Z Starting sccache server...
2020-04-11T22:02:07.2584437Z configure: processing command line
2020-04-11T22:02:07.2584815Z configure: 
2020-04-11T22:02:07.2585741Z configure: rust.parallel-compiler := True
---
2020-04-11T22:05:29.4737080Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T22:05:29.6375324Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T22:05:29.8227173Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T22:05:29.8473291Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T22:05:30.3899440Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T22:05:32.4306641Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T22:05:32.8910975Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T22:05:34.7624208Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T22:05:35.1609485Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T22:07:11.7945165Z configure: rust.codegen-units-std := 1
2020-04-11T22:07:11.7945693Z configure: rust.verify-llvm-ir  := True
2020-04-11T22:07:11.7946106Z configure: llvm.assertions      := True
2020-04-11T22:07:11.7946464Z configure: llvm.ccache          := sccache
2020-04-11T22:07:11.7947136Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-11T22:07:11.7947771Z configure: writing `config.toml` in current directory
2020-04-11T22:07:11.7948004Z configure: 
2020-04-11T22:07:11.7948480Z configure: run `python /checkout/x.py --help`
2020-04-11T22:07:11.7948791Z configure: 
---
2020-04-11T22:08:33.5850462Z Hugepagesize:       2048 kB
2020-04-11T22:08:33.5850640Z DirectMap4k:      141248 kB
2020-04-11T22:08:33.5850816Z DirectMap2M:     5101568 kB
2020-04-11T22:08:33.5851008Z DirectMap1G:     4194304 kB
2020-04-11T22:08:33.5872000Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-11T22:08:34.8288562Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-11T22:08:34.8288562Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-11T22:08:34.8295873Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-11T22:08:35.0374444Z    Compiling unicode-xid v0.2.0
2020-04-11T22:08:35.1571821Z    Compiling syn v1.0.11
2020-04-11T22:08:35.8976914Z    Compiling linked-hash-map v0.5.2
2020-04-11T22:08:35.9376950Z    Compiling lazy_static v1.4.0
2020-04-11T22:08:35.9376950Z    Compiling lazy_static v1.4.0
2020-04-11T22:08:36.1138144Z    Compiling yaml-rust v0.4.3
2020-04-11T22:08:40.0048253Z    Compiling quote v1.0.2
2020-04-11T22:08:52.8307980Z    Compiling thiserror-impl v1.0.5
2020-04-11T22:08:57.0416865Z    Compiling thiserror v1.0.5
2020-04-11T22:08:57.0995473Z    Compiling yaml-merge-keys v0.4.0
2020-04-11T22:08:58.1561126Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-11T22:08:59.5878373Z Build completed successfully in 0:00:25
2020-04-11T22:08:59.5979024Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-11T22:08:59.8370550Z     Finished dev [unoptimized] target(s) in 0.15s
2020-04-11T22:09:00.8529827Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-11T22:10:54.8132825Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T22:10:54.9153620Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T22:10:55.1156189Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T22:10:55.2133577Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T22:10:55.6990929Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T22:10:57.7068296Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T22:10:58.1661269Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T22:11:00.0717548Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T22:11:00.4879331Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T22:14:40.6560870Z Diff in /checkout/src/librustc_mir_build/hair/pattern/const_to_pat.rs at line 110:
2020-04-11T22:14:40.6561202Z                              path, path,
2020-04-11T22:14:40.6561434Z                          )
2020-04-11T22:14:40.6565103Z                      }
2020-04-11T22:14:40.6569111Z -                    traits::NonStructuralMatchTy::Dynamic => {
2020-04-11T22:14:40.6571764Z -                        format!("ok")
2020-04-11T22:14:40.6594026Z -                    }
2020-04-11T22:14:40.6596356Z +                    traits::NonStructuralMatchTy::Dynamic => format!("ok"),
2020-04-11T22:14:40.6596763Z                      traits::NonStructuralMatchTy::Param => {
2020-04-11T22:14:40.6597168Z                          bug!("use of constant whose type is a parameter inside a pattern")
2020-04-11T22:14:40.6597457Z                      }
2020-04-11T22:14:40.6598485Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir_build/hair/pattern/const_to_pat.rs"` failed.
2020-04-11T22:14:40.6599416Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-11T22:14:40.6601710Z Build completed unsuccessfully in 0:00:39
2020-04-11T22:14:40.6690210Z == clock drift check ==
2020-04-11T22:14:40.6707279Z   local time: Sat Apr 11 22:14:40 UTC 2020
2020-04-11T22:14:40.6816121Z   network time: Diff in /checkout/src/librustc_mir_build/hair/pattern/_match.rs at line 316:
2020-04-11T22:14:40.6816121Z   network time: Diff in /checkout/src/librustc_mir_build/hair/pattern/_match.rs at line 316:
2020-04-11T22:14:40.6816518Z              | (_, ty::Str, ty::Str) => val,
2020-04-11T22:14:40.6817350Z              // FIXME(oli-obk): this is reachable for `const FOO: &&&u32 = &&&42;` being used
2020-04-11T22:14:40.6817666Z              (val, _, _) => {
2020-04-11T22:14:40.6818202Z -                self.tcx.sess.delay_span_bug(DUMMY_SP, &format!("cannot deref {:#?}, {} -> {}", val, crty, rty));
2020-04-11T22:14:40.6818569Z +                self.tcx.sess.delay_span_bug(
2020-04-11T22:14:40.6818768Z +                    DUMMY_SP,
2020-04-11T22:14:40.6819200Z +                    &format!("cannot deref {:#?}, {} -> {}", val, crty, rty),
2020-04-11T22:14:40.6819611Z                  val
2020-04-11T22:14:40.6820056Z -            },
2020-04-11T22:14:40.6820208Z +            }
2020-04-11T22:14:40.6820334Z          }
2020-04-11T22:14:40.6820334Z          }
2020-04-11T22:14:40.6820449Z      }
2020-04-11T22:14:40.6820566Z  }
2020-04-11T22:14:40.9784802Z Sat, 11 Apr 2020 22:14:40 GMT
2020-04-11T22:14:42.7169577Z 
2020-04-11T22:14:42.7169577Z 
2020-04-11T22:14:42.7234611Z ##[error]Bash exited with code '1'.
2020-04-11T22:14:42.7247829Z ##[section]Finishing: Run build
2020-04-11T22:14:42.7291453Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71038/merge to s
2020-04-11T22:14:42.7296362Z Task         : Get sources
2020-04-11T22:14:42.7296684Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T22:14:42.7296979Z Version      : 1.0.0
2020-04-11T22:14:42.7297202Z Author       : Microsoft
2020-04-11T22:14:42.7297202Z Author       : Microsoft
2020-04-11T22:14:42.7297530Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T22:14:42.7297916Z ==============================================================================
2020-04-11T22:14:43.0219866Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T22:14:43.0268423Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71038/merge to s
2020-04-11T22:14:43.0344174Z Cleaning up task key
2020-04-11T22:14:43.0345304Z Start cleaning up orphan processes.
2020-04-11T22:14:43.0502248Z Terminate orphan process: pid (3640) (python)
2020-04-11T22:14:43.0627939Z ##[section]Finishing: Finalize Job
