plain
2019-09-28T21:57:29.5570739Z    |
2019-09-28T21:57:29.5570809Z    = note: `#[deny(const_err)]` on by default
2019-09-28T21:57:29.5570860Z 
2019-09-28T21:57:29.5571153Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-28T21:57:29.5571257Z   left: `Size { raw: 0 }`,
2019-09-28T21:57:29.5571589Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-28T21:57:29.5571794Z 
2019-09-28T21:57:29.5571884Z error: internal compiler error: unexpected panic
2019-09-28T21:57:29.5571934Z 
2019-09-28T21:57:29.5572025Z note: the compiler unexpectedly panicked. this is a bug.
---
2019-09-28T21:57:29.5581632Z -LL |     &empty[1..5];
2019-09-28T21:57:29.5581846Z -   |            ^
2019-09-28T21:57:29.5582197Z -   |
2019-09-28T21:57:29.5582521Z -   = note: `-D clippy::out-of-bounds-indexing` implied by `-D warnings`
2019-09-28T21:57:29.5582843Z +thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-28T21:57:29.5582945Z +  left: `Size { raw: 0 }`,
2019-09-28T21:57:29.5583391Z + right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-28T21:57:29.5583502Z +note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-28T21:57:29.5583781Z -error: range is out of bounds
2019-09-28T21:57:29.5583976Z -  --> $DIR/empty_array.rs:8:16
2019-09-28T21:57:29.5584167Z -   |
2019-09-28T21:57:29.5584349Z -LL |     &empty[0..=4];
---
2019-09-28T21:57:29.5588996Z -  --> $DIR/empty_array.rs:12:16
2019-09-28T21:57:29.5589168Z -   |
2019-09-28T21:57:29.5589782Z -LL |     &empty[0..=0];
2019-09-28T21:57:29.5590013Z -   |                ^
2019-09-28T21:57:29.5590293Z +note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-28T21:57:29.5590606Z -error: range is out of bounds
2019-09-28T21:57:29.5591299Z -  --> $DIR/empty_array.rs:13:15
2019-09-28T21:57:29.5591681Z -   |
2019-09-28T21:57:29.5591925Z -LL |     &empty[..=0];
---
2019-09-28T21:57:29.5594565Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'out_of_bounds_indexing/empty_array.rs'
2019-09-28T21:57:29.5594669Z 
2019-09-28T21:57:29.5594749Z error: 1 errors occurred comparing output.
2019-09-28T21:57:29.5594818Z status: exit code: 101
2019-09-28T21:57:29.5596820Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/out_of_bounds_indexing/empty_array.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id.aux" "-A" "unused"
2019-09-28T21:57:29.5598254Z ------------------------------------------
2019-09-28T21:57:29.5598300Z 
2019-09-28T21:57:29.5598519Z ------------------------------------------
2019-09-28T21:57:29.5598597Z stderr:
2019-09-28T21:57:29.5598597Z stderr:
2019-09-28T21:57:29.5598791Z ------------------------------------------
2019-09-28T21:57:29.5600561Z {"message":"index out of bounds: the len is 0 but the index is 0","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/out_of_bounds_indexing/empty_array.rs","byte_start":147,"byte_end":155,"line_start":6,"line_end":6,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    empty[0];","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(const_err)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: index out of bounds: the len is 0 but the index is 0\n  --> tests/ui/out_of_bounds_indexing/empty_array.rs:6:5\n   |\nLL |     empty[0];\n   |     ^^^^^^^^\n   |\n   = note: `#[deny(const_err)]` on by default\n\n"}
2019-09-28T21:57:29.5601219Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-28T21:57:29.5601330Z   left: `Size { raw: 0 }`,
2019-09-28T21:57:29.5601682Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-28T21:57:29.5601889Z 
2019-09-28T21:57:29.5601980Z error: internal compiler error: unexpected panic
2019-09-28T21:57:29.5602034Z 
2019-09-28T21:57:29.5602127Z note: the compiler unexpectedly panicked. this is a bug.
---
2019-09-28T22:40:08.9505848Z Verifying status of rustfmt...
2019-09-28T22:40:08.9518525Z Verifying status of clippy-driver...
2019-09-28T22:40:08.9531352Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-28T22:40:08.9541703Z 
2019-09-28T22:40:08.9542336Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-28T22:40:08.9542569Z 
2019-09-28T22:40:08.9543076Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-28T22:40:08.9543165Z commit another update.
2019-09-28T22:40:08.9543224Z 
2019-09-28T22:40:08.9543501Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-28T22:40:08.9544048Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-28T22:40:08.9544129Z proper steps.
2019-09-28T22:40:08.9568588Z   local time: Sat Sep 28 22:40:08 UTC 2019
2019-09-28T22:40:09.1213881Z   network time: Sat, 28 Sep 2019 22:40:09 GMT
2019-09-28T22:40:09.1215167Z == end clock drift check ==
2019-09-28T22:40:09.1215167Z == end clock drift check ==
2019-09-28T22:40:10.3651480Z ##[error]Bash exited with code '3'.
2019-09-28T22:40:10.3695288Z ##[section]Starting: Upload CPU usage statistics
2019-09-28T22:40:10.3708446Z ==============================================================================
2019-09-28T22:40:10.3708562Z Task         : Bash
2019-09-28T22:40:10.3708636Z Description  : Run a Bash script on macOS, Linux, or Windows
