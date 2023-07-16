plain
2019-08-15T10:11:34.7815383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T10:11:34.7965445Z ##[command]git config gc.auto 0
2019-08-15T10:11:34.8017288Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T10:11:34.8062273Z ##[command]git config --get-all http.proxy
2019-08-15T10:11:34.8178481Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63587/merge:refs/remotes/pull/63587/merge
---
2019-08-15T10:12:09.1929497Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T10:12:09.1930612Z 
2019-08-15T10:12:09.1931656Z   git checkout -b <new-branch-name>
2019-08-15T10:12:09.1932408Z 
2019-08-15T10:12:09.1933127Z HEAD is now at 5e5932bc6 Merge eb30c448de75192b5b511a7b1d9038ae69fcbcd8 into 1cdcea920e56a5d0587307a4c9cf8fff5c77c4bc
2019-08-15T10:12:09.2061287Z ##[section]Finishing: Checkout
2019-08-15T10:12:09.2067547Z ##[section]Starting: Decide whether to run this job
2019-08-15T10:12:09.2070457Z Task         : Bash
2019-08-15T10:12:09.2070504Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-15T10:12:09.2070568Z Version      : 3.151.3
2019-08-15T10:12:09.2070611Z Author       : Microsoft Corporation
2019-08-15T10:12:09.2070611Z Author       : Microsoft Corporation
2019-08-15T10:12:09.2070659Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-15T10:12:09.2070728Z ==============================================================================
2019-08-15T10:12:09.3232231Z Generating script.
2019-08-15T10:12:09.3262778Z ========================== Starting Command Output ===========================
2019-08-15T10:12:09.3282342Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aff1aa2e-0e4d-4249-9d19-3d37b399e653.sh
2019-08-15T10:12:09.4516729Z Executing the job since submodules are updated
2019-08-15T10:12:09.4562610Z ##[section]Finishing: Decide whether to run this job
2019-08-15T10:12:09.4571631Z ==============================================================================
2019-08-15T10:12:09.4571737Z Task         : Bash
2019-08-15T10:12:09.4571786Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-15T10:12:09.4571832Z Version      : 3.151.3
---
2019-08-15T12:07:27.7388262Z failures:
2019-08-15T12:07:27.7388285Z 
2019-08-15T12:07:27.7388505Z ---- [ui] ui-toml/toml_unknown_key/conf_unknown_key.rs stdout ----
2019-08-15T12:07:27.7388548Z normalized stderr:
2019-08-15T12:07:27.7389112Z error: error reading Clippy's configuration file `$DIR/clippy.toml`: unknown field `foobar`, expected one of `blacklisted-names`, `cognitive-complexity-threshold`, `cyclomatic-complexity-threshold`, `doc-valid-idents`, `too-many-arguments-threshold`, `type-complexity-threshold`, `single-char-binding-names-threshold`, `too-large-for-stack`, `enum-variant-name-threshold`, `enum-variant-size-threshold`, `verbose-bit-mask-threshold`, `literal-representation-threshold`, `trivial-copy-size-limit`, `too-many-lines-threshold`, `third-party`
2019-08-15T12:07:27.7389226Z error: aborting due to previous error
2019-08-15T12:07:27.7389263Z 
2019-08-15T12:07:27.7389286Z 
2019-08-15T12:07:27.7389306Z 
2019-08-15T12:07:27.7389306Z 
2019-08-15T12:07:27.7389518Z expected stderr:
2019-08-15T12:07:27.7390118Z error: error reading Clippy's configuration file `$DIR/clippy.toml`: unknown field `foobar`, expected one of `blacklisted-names`, `cognitive-complexity-threshold`, `cyclomatic-complexity-threshold`, `doc-valid-idents`, `too-many-arguments-threshold`, `type-complexity-threshold`, `single-char-binding-names-threshold`, `too-large-for-stack`, `enum-variant-name-threshold`, `enum-variant-size-threshold`, `verbose-bit-mask-threshold`, `literal-representation-threshold`, `trivial-copy-size-limit`, `too-many-lines-threshold`, `third-party` at line 5 column 1
2019-08-15T12:07:27.7390238Z error: aborting due to previous error
2019-08-15T12:07:27.7390260Z 
2019-08-15T12:07:27.7390282Z 
2019-08-15T12:07:27.7390301Z 
2019-08-15T12:07:27.7390301Z 
2019-08-15T12:07:27.7390352Z diff of stderr:
2019-08-15T12:07:27.7390373Z 
2019-08-15T12:07:27.7391049Z -error: error reading Clippy's configuration file `$DIR/clippy.toml`: unknown field `foobar`, expected one of `blacklisted-names`, `cognitive-complexity-threshold`, `cyclomatic-complexity-threshold`, `doc-valid-idents`, `too-many-arguments-threshold`, `type-complexity-threshold`, `single-char-binding-names-threshold`, `too-large-for-stack`, `enum-variant-name-threshold`, `enum-variant-size-threshold`, `verbose-bit-mask-threshold`, `literal-representation-threshold`, `trivial-copy-size-limit`, `too-many-lines-threshold`, `third-party` at line 5 column 1
2019-08-15T12:07:27.7391645Z +error: error reading Clippy's configuration file `$DIR/clippy.toml`: unknown field `foobar`, expected one of `blacklisted-names`, `cognitive-complexity-threshold`, `cyclomatic-complexity-threshold`, `doc-valid-idents`, `too-many-arguments-threshold`, `type-complexity-threshold`, `single-char-binding-names-threshold`, `too-large-for-stack`, `enum-variant-name-threshold`, `enum-variant-size-threshold`, `verbose-bit-mask-threshold`, `literal-representation-threshold`, `trivial-copy-size-limit`, `too-many-lines-threshold`, `third-party`
2019-08-15T12:07:27.7391780Z  error: aborting due to previous error
2019-08-15T12:07:27.7391812Z  
2019-08-15T12:07:27.7391863Z  
2019-08-15T12:07:27.7391884Z 
2019-08-15T12:07:27.7391884Z 
2019-08-15T12:07:27.7391918Z The actual stderr differed from the expected stderr.
2019-08-15T12:07:27.7392243Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/toml_unknown_key/conf_unknown_key.stderr
2019-08-15T12:07:27.7392297Z To update references, run this command from build directory:
2019-08-15T12:07:27.7392628Z /checkout/src/tools/clippy/tests/ui-toml/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base' 'toml_unknown_key/conf_unknown_key.rs'
2019-08-15T12:07:27.7392727Z error: 1 errors occurred comparing output.
2019-08-15T12:07:27.7392761Z status: exit code: 1
2019-08-15T12:07:27.7392761Z status: exit code: 1
2019-08-15T12:07:27.7394443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "/checkout/src/tools/clippy/tests/ui-toml/toml_unknown_key/conf_unknown_key.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/toml_unknown_key/conf_unknown_key.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-982af386409bbff7.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-cba095062173df8b.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-1ce27b71708cd9b5.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/toml_unknown_key/conf_unknown_key.stage-id.aux" "-A" "unused"
2019-08-15T12:07:27.7394993Z ------------------------------------------
2019-08-15T12:07:27.7395029Z 
2019-08-15T12:07:27.7395251Z ------------------------------------------
2019-08-15T12:07:27.7395319Z stderr:
2019-08-15T12:07:27.7395319Z stderr:
2019-08-15T12:07:27.7395536Z ------------------------------------------
2019-08-15T12:07:27.7397177Z {"message":"error reading Clippy's configuration file `/checkout/src/tools/clippy/tests/ui-toml/toml_unknown_key/clippy.toml`: unknown field `foobar`, expected one of `blacklisted-names`, `cognitive-complexity-threshold`, `cyclomatic-complexity-threshold`, `doc-valid-idents`, `too-many-arguments-threshold`, `type-complexity-threshold`, `single-char-binding-names-threshold`, `too-large-for-stack`, `enum-variant-name-threshold`, `enum-variant-size-threshold`, `verbose-bit-mask-threshold`, `literal-representation-threshold`, `trivial-copy-size-limit`, `too-many-lines-threshold`, `third-party`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: error reading Clippy's configuration file `/checkout/src/tools/clippy/tests/ui-toml/toml_unknown_key/clippy.toml`: unknown field `foobar`, expected one of `blacklisted-names`, `cognitive-complexity-threshold`, `cyclomatic-complexity-threshold`, `doc-valid-idents`, `too-many-arguments-threshold`, `type-complexity-threshold`, `single-char-binding-names-threshold`, `too-large-for-stack`, `enum-variant-name-threshold`, `enum-variant-size-threshold`, `verbose-bit-mask-threshold`, `literal-representation-threshold`, `trivial-copy-size-limit`, `too-many-lines-threshold`, `third-party`\n\n"}
2019-08-15T12:07:27.7397576Z 
2019-08-15T12:07:27.7398314Z ------------------------------------------
2019-08-15T12:07:27.7398361Z 
2019-08-15T12:07:27.7398828Z thread '[ui] ui-toml/toml_unknown_key/conf_unknown_key.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
2019-08-15T12:07:28.8797236Z failures:
2019-08-15T12:07:28.8797265Z 
2019-08-15T12:07:28.8797526Z ---- [ui] ui-toml/bad_toml/conf_bad_toml.rs stdout ----
2019-08-15T12:07:28.8797794Z normalized stderr:
2019-08-15T12:07:28.8798208Z error: error reading Clippy's configuration file `$DIR/clippy.toml`: expected an equals, found an identifier at line 1
2019-08-15T12:07:28.8798319Z error: aborting due to previous error
2019-08-15T12:07:28.8798343Z 
2019-08-15T12:07:28.8798363Z 
2019-08-15T12:07:28.8798383Z 
2019-08-15T12:07:28.8798383Z 
2019-08-15T12:07:28.8798415Z expected stderr:
2019-08-15T12:07:28.8798706Z error: error reading Clippy's configuration file `$DIR/clippy.toml`: expected an equals, found an identifier at line 1 column 4
2019-08-15T12:07:28.8798772Z error: aborting due to previous error
2019-08-15T12:07:28.8798812Z 
2019-08-15T12:07:28.8798833Z 
2019-08-15T12:07:28.8798853Z 
2019-08-15T12:07:28.8798853Z 
2019-08-15T12:07:28.8798885Z diff of stderr:
2019-08-15T12:07:28.8798906Z 
2019-08-15T12:07:28.8799163Z -error: error reading Clippy's configuration file `$DIR/clippy.toml`: expected an equals, found an identifier at line 1 column 4
2019-08-15T12:07:28.8799403Z +error: error reading Clippy's configuration file `$DIR/clippy.toml`: expected an equals, found an identifier at line 1
2019-08-15T12:07:28.8799604Z  error: aborting due to previous error
2019-08-15T12:07:28.8799638Z  
2019-08-15T12:07:28.8799668Z  
2019-08-15T12:07:28.8799688Z 
2019-08-15T12:07:28.8799688Z 
2019-08-15T12:07:28.8799742Z The actual stderr differed from the expected stderr.
2019-08-15T12:07:28.8800066Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/bad_toml/conf_bad_toml.stderr
2019-08-15T12:07:28.8800122Z To update references, run this command from build directory:
2019-08-15T12:07:28.8800474Z /checkout/src/tools/clippy/tests/ui-toml/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base' 'bad_toml/conf_bad_toml.rs'
2019-08-15T12:07:28.8800555Z error: 1 errors occurred comparing output.
2019-08-15T12:07:28.8800609Z status: exit code: 1
2019-08-15T12:07:28.8800609Z status: exit code: 1
2019-08-15T12:07:28.8801815Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "/checkout/src/tools/clippy/tests/ui-toml/bad_toml/conf_bad_toml.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/bad_toml/conf_bad_toml.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-982af386409bbff7.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-cba095062173df8b.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-1ce27b71708cd9b5.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/bad_toml/conf_bad_toml.stage-id.aux" "-A" "unused"
2019-08-15T12:07:28.8802185Z ------------------------------------------
2019-08-15T12:07:28.8802213Z 
2019-08-15T12:07:28.8802385Z ------------------------------------------
2019-08-15T12:07:28.8802441Z stderr:
2019-08-15T12:07:28.8802441Z stderr:
2019-08-15T12:07:28.8802612Z ------------------------------------------
2019-08-15T12:07:28.8803062Z {"message":"error reading Clippy's configuration file `/checkout/src/tools/clippy/tests/ui-toml/bad_toml/clippy.toml`: expected an equals, found an identifier at line 1","code":null,"level":"error","spans":[],"children":[],"rendered":"error: error reading Clippy's configuration file `/checkout/src/tools/clippy/tests/ui-toml/bad_toml/clippy.toml`: expected an equals, found an identifier at line 1\n\n"}
2019-08-15T12:07:28.8803262Z 
2019-08-15T12:07:28.8803476Z ------------------------------------------
2019-08-15T12:07:28.8803502Z 
2019-08-15T12:07:28.8804160Z thread '[ui] ui-toml/bad_toml/conf_bad_toml.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
2019-08-15T12:18:08.7544918Z    Compiling rustc-ap-syntax_pos v546.0.0
2019-08-15T12:19:05.4547095Z    Compiling rustc-ap-rustc_errors v546.0.0
2019-08-15T12:23:16.3956037Z    Compiling racer v2.1.25
2019-08-15T12:28:56.0781414Z     Finished release [optimized] target(s) in 21m 26s
2019-08-15T12:28:56.0944833Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-08-15T12:28:56.0945721Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-15T12:28:56.0946192Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-08-15T12:28:56.0946259Z 
2019-08-15T12:28:56.0946311Z the following dependencies are duplicated although they have the same features enabled:
2019-08-15T12:28:56.0946584Z the following dependencies have different features:
2019-08-15T12:28:56.0947202Z   url 2.1.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-08-15T12:28:56.0947538Z     `rls` additionally enabled features {"serde"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-a01c699adf0f8279.rlib"
2019-08-15T12:28:56.0947869Z     `clippy-driver` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-d0557b06f98d57d5.rlib"
2019-08-15T12:28:56.0947906Z 
2019-08-15T12:28:56.0948216Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-08-15T12:28:56.0998834Z Build completed unsuccessfully in 2:12:27
2019-08-15T12:28:56.1058580Z {"rustc-guide":"test-fail","rls":"test-pass","clippy-driver":"test-fail","edition-guide":"test-pass","reference":"test-pass","rustfmt":"test-pass","embedded-book":"test-pass","nomicon":"test-pass","book":"test-pass","miri":"test-pass","rust-by-example":"test-pass","rustbook":"test-fail"}
2019-08-15T12:28:56.1064854Z Verifying status of book...
2019-08-15T12:28:56.1080049Z Verifying status of nomicon...
---
2019-08-15T12:28:56.1139529Z Verifying status of rustfmt...
2019-08-15T12:28:56.1155213Z Verifying status of clippy-driver...
2019-08-15T12:28:56.1171236Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-08-15T12:28:56.1182962Z 
2019-08-15T12:28:56.1183572Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-08-15T12:28:56.1183611Z 
2019-08-15T12:28:56.1183856Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-08-15T12:28:56.1183933Z commit another update.
2019-08-15T12:28:56.1184023Z 
2019-08-15T12:28:56.1184256Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-08-15T12:28:56.1184499Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-08-15T12:28:56.1184550Z proper steps.
2019-08-15T12:28:56.1204706Z   local time: Thu Aug 15 12:28:56 UTC 2019
2019-08-15T12:28:56.2170594Z   network time: Thu, 15 Aug 2019 12:28:56 GMT
2019-08-15T12:28:56.2171226Z == end clock drift check ==
2019-08-15T12:28:56.2171226Z == end clock drift check ==
2019-08-15T12:28:58.3286172Z ##[error]Bash exited with code '3'.
2019-08-15T12:28:58.3327445Z ##[section]Starting: Checkout
2019-08-15T12:28:58.3329015Z ==============================================================================
2019-08-15T12:28:58.3329057Z Task         : Get sources
2019-08-15T12:28:58.3329116Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
