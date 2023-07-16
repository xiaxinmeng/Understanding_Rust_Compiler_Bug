plain
2020-04-25T16:54:02.9522104Z ========================== Starting Command Output ===========================
2020-04-25T16:54:02.9525067Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ed633d74-3c9b-4005-a091-a0a23b8cc160.sh
2020-04-25T16:54:02.9525374Z 
2020-04-25T16:54:02.9529705Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-25T16:54:02.9563295Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-25T16:54:02.9567902Z Task         : Get sources
2020-04-25T16:54:02.9568261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T16:54:02.9568632Z Version      : 1.0.0
2020-04-25T16:54:02.9568866Z Author       : Microsoft
---
2020-04-25T16:54:05.1910643Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-25T16:54:05.2092176Z ##[command]git config gc.auto 0
2020-04-25T16:54:05.2138662Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-25T16:54:05.2160271Z ##[command]git config --get-all http.proxy
2020-04-25T16:54:05.2249790Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71497/merge:refs/remotes/pull/71497/merge
---
2020-04-25T16:57:50.8182188Z  ---> f7353ccad5b1
2020-04-25T16:57:50.8187702Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-25T16:57:50.8189442Z  ---> Using cache
2020-04-25T16:57:50.8190189Z  ---> ed38efbaa060
2020-04-25T16:57:50.8191945Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-25T16:57:50.8194616Z  ---> c5008ef7ae8e
2020-04-25T16:57:50.8227298Z Successfully built c5008ef7ae8e
2020-04-25T16:57:50.8271571Z Successfully tagged rust-ci:latest
2020-04-25T16:57:50.8569436Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-25T16:57:50.8569436Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-25T16:57:50.8587140Z Looks like docker image is the same as before, not uploading
2020-04-25T16:57:59.6234890Z [CI_JOB_NAME=mingw-check]
2020-04-25T16:57:59.6533749Z [CI_JOB_NAME=mingw-check]
2020-04-25T16:57:59.6572473Z == clock drift check ==
2020-04-25T16:57:59.6583511Z   local time: Sat Apr 25 16:57:59 UTC 2020
2020-04-25T16:57:59.9392190Z   network time: Sat, 25 Apr 2020 16:57:59 GMT
2020-04-25T16:57:59.9419191Z Starting sccache server...
2020-04-25T16:58:00.0649717Z configure: processing command line
2020-04-25T16:58:00.0649972Z configure: 
2020-04-25T16:58:00.0650881Z configure: rust.parallel-compiler := True
---
2020-04-25T17:02:15.4907186Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-25T17:02:20.4040634Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-25T17:02:21.7753364Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-25T17:02:21.8994448Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-25T17:02:22.1088730Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-25T17:02:22.9190792Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-25T17:02:23.0776602Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-25T17:02:24.7400715Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-25T17:02:25.2861314Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-25T17:04:32.3310021Z configure: llvm.ccache          := sccache
2020-04-25T17:04:32.3310531Z configure: rust.dist-src        := False
2020-04-25T17:04:32.3310836Z configure: llvm.assertions      := True
2020-04-25T17:04:32.3311134Z configure: build.submodules     := False
2020-04-25T17:04:32.3311777Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-25T17:04:32.3312349Z configure: writing `config.toml` in current directory
2020-04-25T17:04:32.3312802Z configure: 
2020-04-25T17:04:32.3313289Z configure: run `python /checkout/x.py --help`
2020-04-25T17:04:32.3313526Z configure: 
---
2020-04-25T17:06:15.1641748Z Hugepagesize:       2048 kB
2020-04-25T17:06:15.1641952Z DirectMap4k:      147392 kB
2020-04-25T17:06:15.1642158Z DirectMap2M:     4046848 kB
2020-04-25T17:06:15.1642380Z DirectMap1G:     5242880 kB
2020-04-25T17:06:15.1657425Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-25T17:06:16.7097495Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-25T17:06:16.7097495Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-25T17:06:16.7111343Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-25T17:06:16.9776408Z    Compiling unicode-xid v0.2.0
2020-04-25T17:06:17.1118103Z    Compiling syn v1.0.11
2020-04-25T17:06:18.0716447Z    Compiling linked-hash-map v0.5.2
2020-04-25T17:06:18.0939196Z    Compiling lazy_static v1.4.0
2020-04-25T17:06:18.0939196Z    Compiling lazy_static v1.4.0
2020-04-25T17:06:18.3307472Z    Compiling yaml-rust v0.4.3
2020-04-25T17:06:23.2530326Z    Compiling quote v1.0.2
2020-04-25T17:06:39.9546682Z    Compiling thiserror-impl v1.0.5
2020-04-25T17:06:45.3609343Z    Compiling thiserror v1.0.5
2020-04-25T17:06:45.4252229Z    Compiling yaml-merge-keys v0.4.0
2020-04-25T17:06:46.7186417Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-25T17:06:48.5367645Z Build completed successfully in 0:00:33
2020-04-25T17:06:48.5467686Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-25T17:06:48.8699269Z     Finished dev [unoptimized] target(s) in 0.21s
2020-04-25T17:06:50.1573384Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-25T17:09:12.4867358Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-25T17:09:12.7638989Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-25T17:09:12.9725024Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-25T17:09:12.9969275Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-25T17:09:13.6762405Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-25T17:09:16.2453963Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-25T17:09:16.7937977Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-25T17:09:19.0884084Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-25T17:09:19.5869375Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-25T17:13:52.0551679Z Diff in /checkout/src/librustc_codegen_llvm/base.rs at line 23:
2020-04-25T17:13:52.0555834Z  use crate::metadata;
2020-04-25T17:13:52.0559755Z  use crate::value::Value;
2020-04-25T17:13:52.0563563Z  
2020-04-25T17:13:52.0571078Z +use log::debug;
2020-04-25T17:13:52.0595994Z  use rustc_codegen_ssa::base::maybe_create_entry_wrapper;
2020-04-25T17:13:52.0598461Z  use rustc_codegen_ssa::mono_item::MonoItemExt;
2020-04-25T17:13:52.0598863Z  use rustc_codegen_ssa::traits::*;
2020-04-25T17:13:52.0599146Z Diff in /checkout/src/librustc_codegen_llvm/base.rs at line 36:
2020-04-25T17:13:52.0599533Z  use rustc_middle::ty::TyCtxt;
2020-04-25T17:13:52.0600034Z  use rustc_span::symbol::Symbol;
2020-04-25T17:13:52.0600864Z -use log::debug;
2020-04-25T17:13:52.0601216Z  
2020-04-25T17:13:52.0601375Z  use std::ffi::CString;
2020-04-25T17:13:52.0601375Z  use std::ffi::CString;
2020-04-25T17:13:52.0601575Z  use std::time::Instant;
2020-04-25T17:13:52.0601864Z Diff in /checkout/src/librustc_codegen_llvm/base.rs at line 80:
2020-04-25T17:13:52.0602365Z      _tcx: TyCtxt<'tcx>,
2020-04-25T17:13:52.0602588Z      raw_dylibs: &[RawDylibImports],
2020-04-25T17:13:52.0602826Z      llvm_module: &mut ModuleLlvm,
2020-04-25T17:13:52.0603167Z -){
2020-04-25T17:13:52.0603293Z +) {
2020-04-25T17:13:52.0603536Z      let (idata_llctx, idata_llmod) = (&*llvm_module.llcx, llvm_module.llmod());
2020-04-25T17:13:52.0604040Z -
2020-04-25T17:13:52.0604040Z -
2020-04-25T17:13:52.0604244Z      let idata_7 = SmallCStr::new(".idata$7");
2020-04-25T17:13:52.0604544Z      let idata_6 = SmallCStr::new(".idata$6");
2020-04-25T17:13:52.0604965Z Diff in /checkout/src/librustc_codegen_llvm/base.rs at line 121:
2020-04-25T17:13:52.0604965Z Diff in /checkout/src/librustc_codegen_llvm/base.rs at line 121:
2020-04-25T17:13:52.0605315Z                          let llglobal = llvm::LLVMAddGlobal(
2020-04-25T17:13:52.0605611Z                              idata_llmod,
2020-04-25T17:13:52.0605885Z                              common::val_ty(llname),
2020-04-25T17:13:52.0606779Z +                            global_name.as_ptr(),
2020-04-25T17:13:52.0607010Z                          );
2020-04-25T17:13:52.0607179Z  
2020-04-25T17:13:52.0607179Z  
2020-04-25T17:13:52.0607413Z                          llvm::LLVMSetInitializer(llglobal, llname);
2020-04-25T17:13:52.0607757Z Diff in /checkout/src/librustc_codegen_llvm/base.rs at line 128:
2020-04-25T17:13:52.0608097Z                          llvm::LLVMSetGlobalConstant(&llglobal, 1);
2020-04-25T17:13:52.0608521Z                          llvm::LLVMRustSetLinkage(llglobal, llvm::Linkage::PrivateLinkage);
2020-04-25T17:13:52.0608938Z                          llvm::LLVMSetSection(llglobal, idata_6.as_ptr());
2020-04-25T17:13:52.0609788Z -                    _ => {},
2020-04-25T17:13:52.0609988Z +                    }
2020-04-25T17:13:52.0610178Z +                    _ => {}
2020-04-25T17:13:52.0610376Z                  }
---
2020-04-25T17:13:52.0612162Z      }
2020-04-25T17:13:52.0612413Z -
2020-04-25T17:13:52.0612530Z  }
2020-04-25T17:13:52.0612645Z  
2020-04-25T17:13:52.0612971Z  pub struct ValueIter<'ll> {
2020-04-25T17:13:52.0614534Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_codegen_llvm/base.rs"` failed.
2020-04-25T17:13:52.0615535Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-25T17:13:52.0666545Z Build completed unsuccessfully in 0:00:40
2020-04-25T17:13:53.0052419Z == clock drift check ==
2020-04-25T17:13:53.0054993Z   local time: Sat Apr 25 17:13:52 UTC 2020
2020-04-25T17:13:53.0054993Z   local time: Sat Apr 25 17:13:52 UTC 2020
2020-04-25T17:13:53.0055358Z   network time: Sat, 25 Apr 2020 17:13:52 GMT
2020-04-25T17:13:53.9577023Z 
2020-04-25T17:13:53.9577023Z 
2020-04-25T17:13:53.9663892Z ##[error]Bash exited with code '1'.
2020-04-25T17:13:53.9700292Z ##[section]Finishing: Run build
2020-04-25T17:13:53.9757548Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-25T17:13:53.9762575Z Task         : Get sources
2020-04-25T17:13:53.9762937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T17:13:53.9763270Z Version      : 1.0.0
2020-04-25T17:13:53.9763521Z Author       : Microsoft
2020-04-25T17:13:53.9763521Z Author       : Microsoft
2020-04-25T17:13:53.9764071Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-25T17:13:53.9764495Z ==============================================================================
2020-04-25T17:13:54.3711385Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-25T17:13:54.3758691Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-25T17:13:54.3869508Z Cleaning up task key
2020-04-25T17:13:54.3870865Z Start cleaning up orphan processes.
2020-04-25T17:13:54.4099354Z Terminate orphan process: pid (3835) (python)
2020-04-25T17:13:54.4278416Z ##[section]Finishing: Finalize Job
