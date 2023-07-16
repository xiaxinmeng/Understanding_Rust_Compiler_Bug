plain
2019-07-26T02:50:24.1686086Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T02:50:24.1889573Z ##[command]git config gc.auto 0
2019-07-26T02:50:24.1971683Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T02:50:24.2042931Z ##[command]git config --get-all http.proxy
2019-07-26T02:50:24.2174857Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62914/merge:refs/remotes/pull/62914/merge
---
2019-07-26T02:50:58.6267011Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T02:50:58.6267045Z 
2019-07-26T02:50:58.6267282Z   git checkout -b <new-branch-name>
2019-07-26T02:50:58.6267314Z 
2019-07-26T02:50:58.6267382Z HEAD is now at 738c1f0d5 Merge 78f25e308cb5515047f6a3a435b40440ee166e88 into 18630677cf6c7ac50e6786c504b35bc09501dbe2
2019-07-26T02:50:58.6407189Z ##[section]Finishing: Checkout
2019-07-26T02:50:58.6413288Z ##[section]Starting: Decide whether to run this job
2019-07-26T02:50:58.6416450Z Task         : Bash
2019-07-26T02:50:58.6416499Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-26T02:50:58.6416564Z Version      : 3.151.3
2019-07-26T02:50:58.6416610Z Author       : Microsoft Corporation
2019-07-26T02:50:58.6416610Z Author       : Microsoft Corporation
2019-07-26T02:50:58.6416661Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-26T02:50:58.6416731Z ==============================================================================
2019-07-26T02:50:58.7796496Z Generating script.
2019-07-26T02:50:58.7827405Z ========================== Starting Command Output ===========================
2019-07-26T02:50:58.7853086Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/85d6897f-d414-4d4f-ab3c-bc84912bab37.sh
2019-07-26T02:50:59.0730801Z Executing the job since submodules are updated
2019-07-26T02:50:59.0839206Z ##[section]Finishing: Decide whether to run this job
2019-07-26T02:50:59.0855075Z ==============================================================================
2019-07-26T02:50:59.0855140Z Task         : Bash
2019-07-26T02:50:59.0855233Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-26T02:50:59.0855279Z Version      : 3.151.3
---
2019-07-26T04:54:18.3649170Z normalized stderr:
2019-07-26T04:54:18.3649289Z error: unnecessary `unsafe` block
2019-07-26T04:54:18.3650107Z   --> $DIR/intrinsics.rs:14:16
2019-07-26T04:54:18.3650394Z    |
2019-07-26T04:54:18.3650460Z 14 |     assert_eq!(unsafe { type_name::<Option<i32>>() }, "core::option::Option<i32>");
2019-07-26T04:54:18.3650536Z    |                ^^^^^^ unnecessary `unsafe` block
2019-07-26T04:54:18.3650915Z    = note: `-D unused-unsafe` implied by `-D unused`
2019-07-26T04:54:18.3650955Z 
2019-07-26T04:54:18.3651019Z error: aborting due to previous error
2019-07-26T04:54:18.3651050Z 
---
2019-07-26T04:54:18.3662233Z 
2019-07-26T04:54:18.3667886Z +error: unnecessary `unsafe` block
2019-07-26T04:54:18.3668350Z +  --> $DIR/intrinsics.rs:14:16
2019-07-26T04:54:18.3673542Z +   |
2019-07-26T04:54:18.3673642Z +14 |     assert_eq!(unsafe { type_name::<Option<i32>>() }, "core::option::Option<i32>");
2019-07-26T04:54:18.3681492Z +   |                ^^^^^^ unnecessary `unsafe` block
2019-07-26T04:54:18.3700903Z +   = note: `-D unused-unsafe` implied by `-D unused`
2019-07-26T04:54:18.3701337Z thread '[ui] run-pass/intrinsics.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-26T04:54:18.3701416Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-26T04:54:18.3701491Z +
2019-07-26T04:54:18.3701491Z +
2019-07-26T04:54:18.3701537Z +error: aborting due to previous error
2019-07-26T04:54:18.3701581Z +
2019-07-26T04:54:18.3701639Z +
2019-07-26T04:54:18.3701667Z 
2019-07-26T04:54:18.3701715Z The actual stderr differed from the expected stderr.
2019-07-26T04:54:18.3701786Z Actual stderr saved to /tmp/compiletestX9olrk/intrinsics.stderr
2019-07-26T04:54:18.3701838Z To update references, run this command from build directory:
2019-07-26T04:54:18.3702214Z tests/run-pass/update-references.sh '/tmp/compiletestX9olrk' 'intrinsics.rs'
2019-07-26T04:54:18.3702347Z error: 1 errors occurred comparing output.
2019-07-26T04:54:18.3702392Z status: exit code: 1
2019-07-26T04:54:18.3702392Z status: exit code: 1
2019-07-26T04:54:18.3703044Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestX9olrk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestX9olrk/intrinsics.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestX9olrk/intrinsics.stage-id.aux" "-A" "unused"
2019-07-26T04:54:18.3703407Z ------------------------------------------
2019-07-26T04:54:18.3703445Z 
2019-07-26T04:54:18.3703717Z ------------------------------------------
2019-07-26T04:54:18.3703765Z stderr:
2019-07-26T04:54:18.3703765Z stderr:
2019-07-26T04:54:18.3704007Z ------------------------------------------
2019-07-26T04:54:18.3705186Z {"message":"unnecessary `unsafe` block","code":{"code":"unused_unsafe","explanation":null},"level":"error","spans":[{"file_name":"tests/run-pass/intrinsics.rs","byte_start":392,"byte_end":398,"line_start":14,"line_end":14,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"    assert_eq!(unsafe { type_name::<Option<i32>>() }, \"core::option::Option<i32>\");","highlight_start":16,"highlight_end":22}],"label":"unnecessary `unsafe` block","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unused-unsafe` implied by `-D unused`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unnecessary `unsafe` block\n  --> tests/run-pass/intrinsics.rs:14:16\n   |\n14 |     assert_eq!(unsafe { type_name::<Option<i32>>() }, \"core::option::Option<i32>\");\n   |                ^^^^^^ unnecessary `unsafe` block\n   |\n   = note: `-D unused-unsafe` implied by `-D unused`\n\n"}
2019-07-26T04:54:18.3705573Z 
2019-07-26T04:54:18.3705881Z ------------------------------------------
2019-07-26T04:54:18.3705917Z 
2019-07-26T04:54:18.3706165Z test [ui] run-pass/intrinsics.rs ... FAILED
---
2019-07-26T04:54:33.8729053Z Cloning into 'rust-toolstate'...
2019-07-26T04:54:34.5213214Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-26T04:54:34.5582765Z [master 1edfcd0] (linux CI update)
2019-07-26T04:54:34.5582970Z  1 file changed, 1 insertion(+)
2019-07-26T04:54:35.1893321Z remote: Invalid username or password.
2019-07-26T04:54:35.1895985Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-26T04:54:38.5170422Z  * branch            master     -> FETCH_HEAD
2019-07-26T04:54:38.5373069Z HEAD is now at d482fe9 (linux CI update)
2019-07-26T04:54:38.5562874Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-26T04:54:38.5764986Z [master a4f8d83] (linux CI update)
2019-07-26T04:54:38.5764986Z [master a4f8d83] (linux CI update)
2019-07-26T04:54:38.5766655Z  1 file changed, 1 insertion(+)
2019-07-26T04:54:38.8623825Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-26T04:54:41.1762041Z  * branch            master     -> FETCH_HEAD
2019-07-26T04:54:41.1916372Z HEAD is now at d482fe9 (linux CI update)
2019-07-26T04:54:41.2027486Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-26T04:54:41.2240652Z [master 77c44f8] (linux CI update)
2019-07-26T04:54:41.2240652Z [master 77c44f8] (linux CI update)
2019-07-26T04:54:41.2240892Z  1 file changed, 1 insertion(+)
2019-07-26T04:54:41.5163825Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-26T04:54:44.8383666Z  * branch            master     -> FETCH_HEAD
2019-07-26T04:54:44.8572415Z HEAD is now at d482fe9 (linux CI update)
2019-07-26T04:54:44.8775688Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-26T04:54:44.9027341Z [master 09cfe89] (linux CI update)
2019-07-26T04:54:44.9027341Z [master 09cfe89] (linux CI update)
2019-07-26T04:54:44.9028192Z  1 file changed, 1 insertion(+)
2019-07-26T04:54:45.2072039Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-26T04:54:48.5313348Z  * branch            master     -> FETCH_HEAD
2019-07-26T04:54:48.5543661Z HEAD is now at d482fe9 (linux CI update)
2019-07-26T04:54:48.5657618Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-26T04:54:48.5850443Z [master 27868b2] (linux CI update)
2019-07-26T04:54:48.5850443Z [master 27868b2] (linux CI update)
2019-07-26T04:54:48.5851278Z  1 file changed, 1 insertion(+)
2019-07-26T04:54:48.8617794Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-26T04:54:50.1705081Z  * branch            master     -> FETCH_HEAD
2019-07-26T04:54:50.1869729Z HEAD is now at d482fe9 (linux CI update)
2019-07-26T04:54:50.1869729Z HEAD is now at d482fe9 (linux CI update)
2019-07-26T04:54:50.9099131Z ##[error]Bash exited with code '1'.
2019-07-26T04:54:50.9140711Z ##[section]Starting: Checkout
2019-07-26T04:54:50.9142525Z ==============================================================================
2019-07-26T04:54:50.9142579Z Task         : Get sources
2019-07-26T04:54:50.9142627Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
