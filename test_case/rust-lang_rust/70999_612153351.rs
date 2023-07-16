plain
2020-04-10T18:07:18.8333854Z ========================== Starting Command Output ===========================
2020-04-10T18:07:18.8336258Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a64b6316-9517-48af-b81a-6e1f8db8be63.sh
2020-04-10T18:07:18.8336523Z 
2020-04-10T18:07:18.8340399Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T18:07:18.8361346Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T18:07:18.8365960Z Task         : Get sources
2020-04-10T18:07:18.8366262Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T18:07:18.8366571Z Version      : 1.0.0
2020-04-10T18:07:18.8366770Z Author       : Microsoft
---
2020-04-10T18:07:21.5692662Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T18:07:21.5750445Z ##[command]git config gc.auto 0
2020-04-10T18:07:21.5754606Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T18:07:21.5758674Z ##[command]git config --get-all http.proxy
2020-04-10T18:07:21.5767079Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70999/merge:refs/remotes/pull/70999/merge
---
2020-04-10T18:11:17.6524223Z  ---> 3fc1b512c57b
2020-04-10T18:11:17.6524780Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-10T18:11:17.6525265Z  ---> Using cache
2020-04-10T18:11:17.6525676Z  ---> 5ee4295733f4
2020-04-10T18:11:17.6527499Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-10T18:11:17.6529201Z  ---> 3d07a0fa42fe
2020-04-10T18:11:17.6595110Z Successfully built 3d07a0fa42fe
2020-04-10T18:11:17.6628124Z Successfully tagged rust-ci:latest
2020-04-10T18:11:17.7328281Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T18:11:17.7328281Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T18:11:17.7342977Z Looks like docker image is the same as before, not uploading
2020-04-10T18:11:26.5810640Z [CI_JOB_NAME=mingw-check]
2020-04-10T18:11:26.6030404Z [CI_JOB_NAME=mingw-check]
2020-04-10T18:11:26.6064071Z == clock drift check ==
2020-04-10T18:11:26.6072828Z   local time: Fri Apr 10 18:11:26 UTC 2020
2020-04-10T18:11:26.9200101Z   network time: Fri, 10 Apr 2020 18:11:26 GMT
2020-04-10T18:11:26.9223493Z Starting sccache server...
2020-04-10T18:11:27.0028146Z configure: processing command line
2020-04-10T18:11:27.0028788Z configure: 
2020-04-10T18:11:27.0029798Z configure: rust.parallel-compiler := True
---
2020-04-10T18:14:27.7212137Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-10T18:14:28.4999660Z error: unused extern crate
2020-04-10T18:14:28.5000444Z   --> src/librustc_data_structures/lib.rs:30:1
2020-04-10T18:14:28.5000943Z    |
2020-04-10T18:14:28.5001511Z 29 | / #[cfg(unix)]
2020-04-10T18:14:28.5002231Z 30 | | extern crate libc;
2020-04-10T18:14:28.5002969Z    | | ^^^^^^^^^^^^^^^^^-
2020-04-10T18:14:28.5004264Z    |                    help: remove it
2020-04-10T18:14:28.5004779Z    |
2020-04-10T18:14:28.5005364Z    = note: `-D unused-extern-crates` implied by `-D warnings`
2020-04-10T18:14:28.5005675Z 
2020-04-10T18:14:28.5005675Z 
2020-04-10T18:14:28.5039475Z error: aborting due to previous error
2020-04-10T18:14:28.5039716Z 
2020-04-10T18:14:28.5092468Z error: could not compile `rustc_data_structures`.
2020-04-10T18:14:28.5092734Z 
2020-04-10T18:14:28.5093125Z To learn more, run the command again with --verbose.
2020-04-10T18:14:28.5093677Z warning: build failed, waiting for other jobs to finish...
2020-04-10T18:14:37.7568053Z error: build failed
2020-04-10T18:14:37.7596457Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--no-default-features" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-10T18:14:37.7610814Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-10T18:14:37.7611498Z Build completed unsuccessfully in 0:03:10
2020-04-10T18:14:37.7659193Z == clock drift check ==
2020-04-10T18:14:37.7659193Z == clock drift check ==
2020-04-10T18:14:37.7682642Z   local time: Fri Apr 10 18:14:37 UTC 2020
2020-04-10T18:14:37.8581106Z   network time: Fri, 10 Apr 2020 18:14:37 GMT
2020-04-10T18:14:38.4476725Z 
2020-04-10T18:14:38.4476725Z 
2020-04-10T18:14:38.4537441Z ##[error]Bash exited with code '1'.
2020-04-10T18:14:38.4548705Z ##[section]Finishing: Run build
2020-04-10T18:14:38.4602172Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T18:14:38.4606575Z Task         : Get sources
2020-04-10T18:14:38.4606860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T18:14:38.4607121Z Version      : 1.0.0
2020-04-10T18:14:38.4607342Z Author       : Microsoft
2020-04-10T18:14:38.4607342Z Author       : Microsoft
2020-04-10T18:14:38.4607634Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T18:14:38.4607978Z ==============================================================================
2020-04-10T18:14:38.7554585Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T18:14:38.7598525Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T18:14:38.7674474Z Cleaning up task key
2020-04-10T18:14:38.7675614Z Start cleaning up orphan processes.
2020-04-10T18:14:38.7894276Z Terminate orphan process: pid (4088) (python)
2020-04-10T18:14:38.8200022Z ##[section]Finishing: Finalize Job
