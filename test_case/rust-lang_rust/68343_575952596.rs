plain
2020-01-18T23:59:21.2985063Z test [ui] ui/path_buf_push_overwrite.rs ... ok
2020-01-18T23:59:21.3945592Z 
2020-01-18T23:59:21.3949974Z error: failed to compile fixed code
2020-01-18T23:59:21.3953067Z status: exit code: 1
2020-01-18T23:59:21.3963984Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/patterns.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-50e1ee08d654e11f/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-50e1ee08d654e11f/out/test_build_base/patterns.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e872394251f07f48.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-2b50a9c81f92b99d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-2cea9c273f40f029.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-50e1ee08d654e11f/out/test_build_base/patterns.stage-id.aux"
2020-01-18T23:59:21.3965123Z ------------------------------------------
2020-01-18T23:59:21.3969035Z 
2020-01-18T23:59:21.3969610Z ------------------------------------------
2020-01-18T23:59:21.3969676Z stderr:
2020-01-18T23:59:21.3969676Z stderr:
2020-01-18T23:59:21.3969895Z ------------------------------------------
2020-01-18T23:59:21.3975957Z {"message":"the feature `slice_patterns` has been stable since 1.42.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/patterns.fixed","byte_start":66,"byte_end":80,"line_start":4,"line_end":4,"column_start":12,"column_end":26,"is_primary":true,"text":[{"text":"#![feature(slice_patterns)]","highlight_start":12,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `slice_patterns` has been stable since 1.42.0 and no longer requires an attribute to enable\n  --> tests/ui/patterns.fixed:4:12\n   |\nLL | #![feature(slice_patterns)]\n   |            ^^^^^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
2020-01-18T23:59:21.3981056Z 
2020-01-18T23:59:21.3981463Z ------------------------------------------
2020-01-18T23:59:21.3981506Z 
2020-01-18T23:59:21.3989951Z test [ui] ui/patterns.rs ... FAILED
---
2020-01-19T00:36:43.7015358Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2020-01-19T00:36:43.7015415Z 
2020-01-19T00:36:43.7015851Z We detected that this PR updated 'clippy-driver', but its tests failed.
2020-01-19T00:36:43.7015905Z 
2020-01-19T00:36:43.7016182Z If you do intend to update 'clippy-driver', please check the error messages above and
2020-01-19T00:36:43.7016275Z commit another update.
2020-01-19T00:36:43.7016331Z 
2020-01-19T00:36:43.7016942Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2020-01-19T00:36:43.7017238Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2020-01-19T00:36:43.7017671Z proper steps.
2020-01-19T00:36:43.7021579Z Build completed unsuccessfully in 0:00:01
2020-01-19T00:36:43.7069809Z == clock drift check ==
2020-01-19T00:36:43.7085350Z   local time: Sun Jan 19 00:36:43 UTC 2020
2020-01-19T00:36:44.2370363Z   network time: Sun, 19 Jan 2020 00:36:44 GMT
2020-01-19T00:36:44.2370363Z   network time: Sun, 19 Jan 2020 00:36:44 GMT
2020-01-19T00:36:44.2370477Z == end clock drift check ==
2020-01-19T00:36:45.5469573Z 
2020-01-19T00:36:45.5562701Z ##[error]Bash exited with code '1'.
2020-01-19T00:36:45.5598256Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-19T00:36:45.5600381Z ==============================================================================
2020-01-19T00:36:45.5600476Z Task         : Get sources
2020-01-19T00:36:45.5600579Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
