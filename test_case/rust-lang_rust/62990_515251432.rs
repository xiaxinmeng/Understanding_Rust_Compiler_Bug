plain
2019-07-25T21:21:47.0287180Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T21:21:47.0440026Z ##[command]git config gc.auto 0
2019-07-25T21:21:47.0512074Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T21:21:47.0590553Z ##[command]git config --get-all http.proxy
2019-07-25T21:21:47.0709344Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62990/merge:refs/remotes/pull/62990/merge
---
2019-07-25T21:22:22.0626222Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T21:22:22.0626255Z 
2019-07-25T21:22:22.0626475Z   git checkout -b <new-branch-name>
2019-07-25T21:22:22.0626527Z 
2019-07-25T21:22:22.0626576Z HEAD is now at 94c85901c Merge 1a775b3b55263a82c3e69a942de5d2cdfb374bf1 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T21:22:22.0752839Z ##[section]Finishing: Checkout
2019-07-25T21:22:22.0759788Z ##[section]Starting: Decide whether to run this job
2019-07-25T21:22:22.0762355Z Task         : Bash
2019-07-25T21:22:22.0762392Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T21:22:22.0762445Z Version      : 3.151.3
2019-07-25T21:22:22.0762480Z Author       : Microsoft Corporation
2019-07-25T21:22:22.0762480Z Author       : Microsoft Corporation
2019-07-25T21:22:22.0762519Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-25T21:22:22.0762577Z ==============================================================================
2019-07-25T21:22:22.2056004Z Generating script.
2019-07-25T21:22:22.2086951Z ========================== Starting Command Output ===========================
2019-07-25T21:22:22.2102716Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/73df683b-4b1d-4abb-9399-7c9c0630edb8.sh
2019-07-25T21:22:22.3161394Z Executing the job since submodules are updated
2019-07-25T21:22:22.3243634Z ##[section]Finishing: Decide whether to run this job
2019-07-25T21:22:22.3252779Z ==============================================================================
2019-07-25T21:22:22.3252867Z Task         : Bash
2019-07-25T21:22:22.3252904Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T21:22:22.3252938Z Version      : 3.151.3
---
2019-07-25T23:26:39.9593338Z test [ui] run-pass/intrinsics-math.rs ... ok
2019-07-25T23:26:39.9952916Z thread '[ui] run-pass/intrinsics.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-25T23:26:39.9953750Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T23:26:39.9954354Z normalized stderr:
2019-07-25T23:26:39.9954426Z error: unnecessary `unsafe` block
2019-07-25T23:26:39.9955047Z    |
2019-07-25T23:26:39.9955047Z    |
2019-07-25T23:26:39.9956522Z 14 |     assert_eq!(unsafe { type_name::<Option<i32>>() }, "core::option::Option<i32>");
2019-07-25T23:26:39.9956860Z    |                ^^^^^^ unnecessary `unsafe` block
2019-07-25T23:26:39.9956995Z    |
2019-07-25T23:26:39.9957720Z    = note: `-D unused-unsafe` implied by `-D unused`
2019-07-25T23:26:39.9957814Z error: aborting due to previous error
2019-07-25T23:26:39.9957877Z 
2019-07-25T23:26:39.9957899Z 
2019-07-25T23:26:39.9957919Z 
2019-07-25T23:26:39.9957919Z 
2019-07-25T23:26:39.9958358Z expected stderr:
2019-07-25T23:26:39.9958397Z 
2019-07-25T23:26:39.9958442Z 
2019-07-25T23:26:39.9958486Z diff of stderr:
2019-07-25T23:26:39.9958515Z 
2019-07-25T23:26:39.9958559Z +error: unnecessary `unsafe` block
2019-07-25T23:26:39.9958877Z +  --> $DIR/intrinsics.rs:14:16
2019-07-25T23:26:39.9958928Z +   |
2019-07-25T23:26:39.9959025Z +14 |     assert_eq!(unsafe { type_name::<Option<i32>>() }, "core::option::Option<i32>");
2019-07-25T23:26:39.9959103Z +   |                ^^^^^^ unnecessary `unsafe` block
2019-07-25T23:26:39.9959374Z +   |
2019-07-25T23:26:39.9959665Z +   = note: `-D unused-unsafe` implied by `-D unused`
2019-07-25T23:26:39.9959782Z +error: aborting due to previous error
2019-07-25T23:26:39.9959825Z +
2019-07-25T23:26:39.9959865Z +
2019-07-25T23:26:39.9959912Z 
2019-07-25T23:26:39.9959912Z 
2019-07-25T23:26:39.9959960Z The actual stderr differed from the expected stderr.
2019-07-25T23:26:39.9960022Z Actual stderr saved to /tmp/compiletestgrkXW1/intrinsics.stderr
2019-07-25T23:26:39.9960093Z To update references, run this command from build directory:
2019-07-25T23:26:39.9960373Z tests/run-pass/update-references.sh '/tmp/compiletestgrkXW1' 'intrinsics.rs'
2019-07-25T23:26:39.9960456Z error: 1 errors occurred comparing output.
2019-07-25T23:26:39.9960522Z status: exit code: 1
2019-07-25T23:26:39.9960522Z status: exit code: 1
2019-07-25T23:26:39.9961147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestgrkXW1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestgrkXW1/intrinsics.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestgrkXW1/intrinsics.stage-id.aux" "-A" "unused"
2019-07-25T23:26:39.9961490Z ------------------------------------------
2019-07-25T23:26:39.9961536Z 
2019-07-25T23:26:39.9961927Z ------------------------------------------
2019-07-25T23:26:39.9961965Z stderr:
2019-07-25T23:26:39.9961965Z stderr:
2019-07-25T23:26:39.9962143Z ------------------------------------------
2019-07-25T23:26:39.9963169Z {"message":"unnecessary `unsafe` block","code":{"code":"unused_unsafe","explanation":null},"level":"error","spans":[{"file_name":"tests/run-pass/intrinsics.rs","byte_start":392,"byte_end":398,"line_start":14,"line_end":14,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"    assert_eq!(unsafe { type_name::<Option<i32>>() }, \"core::option::Option<i32>\");","highlight_start":16,"highlight_end":22}],"label":"unnecessary `unsafe` block","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unused-unsafe` implied by `-D unused`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unnecessary `unsafe` block\n  --> tests/run-pass/intrinsics.rs:14:16\n   |\n14 |     assert_eq!(unsafe { type_name::<Option<i32>>() }, \"core::option::Option<i32>\");\n   |                ^^^^^^ unnecessary `unsafe` block\n   |\n   = note: `-D unused-unsafe` implied by `-D unused`\n\n"}
2019-07-25T23:26:39.9963347Z 
2019-07-25T23:26:39.9963564Z ------------------------------------------
2019-07-25T23:26:39.9963609Z 
2019-07-25T23:26:39.9963974Z test [ui] run-pass/intrinsics.rs ... FAILED
---
2019-07-25T23:26:54.5216160Z The state of "rustfmt" has changed from "build-fail" to "test-pass"
2019-07-25T23:26:54.5216361Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T23:26:54.5439063Z [master dd452fc] (linux CI update)
2019-07-25T23:26:54.5439213Z  1 file changed, 1 insertion(+)
2019-07-25T23:26:55.1654369Z remote: Invalid username or password.
2019-07-25T23:26:55.1655825Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-25T23:26:55.4681910Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:26:55.4681910Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:26:55.4840537Z HEAD is now at 37b0f83 📣 Toolstate changed by rust-lang/rust#60340!
2019-07-25T23:26:55.4941836Z The state of "rustfmt" has changed from "build-fail" to "test-pass"
2019-07-25T23:26:55.4942114Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T23:26:55.5099964Z [master 70ee3ea] (linux CI update)
2019-07-25T23:26:55.5100349Z  1 file changed, 1 insertion(+)
2019-07-25T23:26:55.5100349Z  1 file changed, 1 insertion(+)
2019-07-25T23:26:55.7890819Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T23:26:59.1252331Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:26:59.1252331Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:26:59.1395625Z HEAD is now at 37b0f83 📣 Toolstate changed by rust-lang/rust#60340!
2019-07-25T23:26:59.1499956Z The state of "rustfmt" has changed from "build-fail" to "test-pass"
2019-07-25T23:26:59.1500887Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T23:26:59.1672438Z [master 75b1a68] (linux CI update)
2019-07-25T23:26:59.1672866Z  1 file changed, 1 insertion(+)
2019-07-25T23:26:59.1672866Z  1 file changed, 1 insertion(+)
2019-07-25T23:26:59.4606849Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T23:27:03.1673184Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:27:03.1673184Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:27:03.1852207Z HEAD is now at 37b0f83 📣 Toolstate changed by rust-lang/rust#60340!
2019-07-25T23:27:03.1952569Z The state of "rustfmt" has changed from "build-fail" to "test-pass"
2019-07-25T23:27:03.1953217Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T23:27:03.2107139Z [master 7ed82f9] (linux CI update)
2019-07-25T23:27:03.2107473Z  1 file changed, 1 insertion(+)
2019-07-25T23:27:03.2107473Z  1 file changed, 1 insertion(+)
2019-07-25T23:27:03.6891902Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T23:27:05.0163131Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:27:05.0163131Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:27:05.0310728Z HEAD is now at 37b0f83 📣 Toolstate changed by rust-lang/rust#60340!
2019-07-25T23:27:05.0414581Z The state of "rustfmt" has changed from "build-fail" to "test-pass"
2019-07-25T23:27:05.0415017Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T23:27:05.0582864Z [master 5d3a1f8] (linux CI update)
2019-07-25T23:27:05.0582925Z  1 file changed, 1 insertion(+)
2019-07-25T23:27:05.0582925Z  1 file changed, 1 insertion(+)
2019-07-25T23:27:05.3701077Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T23:27:06.6877831Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:27:06.6877831Z  * branch            master     -> FETCH_HEAD
2019-07-25T23:27:06.7013445Z HEAD is now at 37b0f83 📣 Toolstate changed by rust-lang/rust#60340!
2019-07-25T23:27:10.8123857Z ##[error]Bash exited with code '1'.
2019-07-25T23:27:10.8157039Z ##[section]Starting: Checkout
2019-07-25T23:27:10.8159111Z ==============================================================================
2019-07-25T23:27:10.8159186Z Task         : Get sources
2019-07-25T23:27:10.8159236Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
