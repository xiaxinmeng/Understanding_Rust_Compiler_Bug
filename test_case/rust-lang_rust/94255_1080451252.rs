plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 13c9fc38c94969ce4b91615bc803d923be8e0f51 and f90dd4e060d771e22c587fcf36dbfd0ac2b6d6ec
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
To only update this specific test, also pass `--test-args concurrency/libc_pthread_cond.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/libc_pthread_cond.rs" "-L" "/tmp/compiletestMM0kAH" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMM0kAH/concurrency/libc_pthread_cond.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestMM0kAH/concurrency/libc_pthread_cond.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args concurrency/sync.rs`

error: 2 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/sync.rs" "-L" "/tmp/compiletestMM0kAH" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMM0kAH/concurrency/sync.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestMM0kAH/concurrency/sync.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestMM0kAH/vec.stage-id.stderr
To only update this specific test, also pass `--test-args vec.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec.rs" "-L" "/tmp/compiletestMM0kAH" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMM0kAH/vec.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestMM0kAH/vec.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

 error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:13:9
    |
 LL |         0..=10 => println!("0..=10"),
    |
    |
    = note: `-D clippy::match-overlapping-arm` implied by `-D warnings`
 note: overlaps with this
    |
    |
 LL |         0..=11 => println!("0..=11"),
 
 error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:19:9
    |
    |
 LL |         0..=5 => println!("0..=5"),
    |
 note: overlaps with this
   --> $DIR/match_overlapping_arm.rs:21:9
    |
    |
 LL |         FOO..=11 => println!("FOO..=11"),
 
 error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:56:9
    |
    |
 LL |         0..11 => println!("0..11"),
    |
 note: overlaps with this
   --> $DIR/match_overlapping_arm.rs:57:9
    |
    |
 LL |         0..=11 => println!("0..=11"),
 
 error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:81:9
    |
    |
 LL |         0..=10 => println!("0..=10"),
    |
 note: overlaps with this
   --> $DIR/match_overlapping_arm.rs:80:9
    |
    |
 LL |         5..14 => println!("5..14"),
 
 error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:86:9
    |
    |
 LL |         0..7 => println!("0..7"),
    |
 note: overlaps with this
   --> $DIR/match_overlapping_arm.rs:87:9
    |
    |
 LL |         0..=10 => println!("0..=10"),
 
 error: some ranges overlap
-  --> $DIR/match_overlapping_arm.rs:98:9
-   |
-   |
-LL |         ..=23 => println!("..=23"),
-   |         ^^^^^
-   |
-note: overlaps with this
-   |
-   |
-LL |         ..26 => println!("..26"),
-   |         ^^^^
-error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:107:9
    |
    |
 LL |         21..=30 => (),
    |
 note: overlaps with this
   --> $DIR/match_overlapping_arm.rs:108:9
    |
    |
 LL |         21..=40 => (),
 
 error: some ranges overlap
   --> $DIR/match_overlapping_arm.rs:121:9
    |
    |
 LL |         0..=0x0000_0000_0000_00ff => (),
error: test failed, to rerun pass '--test compile-test'
    |
 note: overlaps with this
   --> $DIR/match_overlapping_arm.rs:122:9
   --> $DIR/match_overlapping_arm.rs:122:9
    |
 LL |         0..=0x0000_0000_0000_ffff => (),
 
-error: aborting due to 8 previous errors
+error: aborting due to 7 previous errors
 
---
To only update this specific test, also pass `--test-args match_overlapping_arm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/match_overlapping_arm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/match_overlapping_arm.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/match_overlapping_arm.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":329,"byte_end":335,"line_start":13,"line_end":13,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        0..=10 => println!(\"0..=10\"),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-overlapping-arm` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":367,"byte_end":373,"line_start":14,"line_end":14,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        0..=11 => println!(\"0..=11\"),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:13:9\n   |\nLL |         0..=10 => println!(\"0..=10\"),\n   |         ^^^^^^\n   |\n   = note: `-D clippy::match-overlapping-arm` implied by `-D warnings`\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:14:9\n   |\nLL |         0..=11 => println!(\"0..=11\"),\n   |         ^^^^^^\n\n"}
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":444,"byte_end":449,"line_start":19,"line_end":19,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        0..=5 => println!(\"0..=5\"),","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":516,"byte_end":524,"line_start":21,"line_end":21,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        FOO..=11 => println!(\"FOO..=11\"),","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:19:9\n   |\nLL |         0..=5 => println!(\"0..=5\"),\n   |         ^^^^^\n   |\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:21:9\n   |\nLL |         FOO..=11 => println!(\"FOO..=11\"),\n   |         ^^^^^^^^\n\n"}
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":1134,"byte_end":1139,"line_start":56,"line_end":56,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        0..11 => println!(\"0..11\"),","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":1170,"byte_end":1176,"line_start":57,"line_end":57,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        0..=11 => println!(\"0..=11\"),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:56:9\n   |\nLL |         0..11 => println!(\"0..11\"),\n   |         ^^^^^\n   |\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:57:9\n   |\nLL |         0..=11 => println!(\"0..=11\"),\n   |         ^^^^^^\n\n"}
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":1616,"byte_end":1622,"line_start":81,"line_end":81,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        0..=10 => println!(\"0..=10\"),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":1580,"byte_end":1585,"line_start":80,"line_end":80,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        5..14 => println!(\"5..14\"),","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:81:9\n   |\nLL |         0..=10 => println!(\"0..=10\"),\n   |         ^^^^^^\n   |\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:80:9\n   |\nLL |         5..14 => println!(\"5..14\"),\n   |         ^^^^^\n\n"}
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":1693,"byte_end":1697,"line_start":86,"line_end":86,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        0..7 => println!(\"0..7\"),","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":1727,"byte_end":1733,"line_start":87,"line_end":87,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        0..=10 => println!(\"0..=10\"),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:86:9\n   |\nLL |         0..7 => println!(\"0..7\"),\n   |         ^^^^\n   |\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:87:9\n   |\nLL |         0..=10 => println!(\"0..=10\"),\n   |         ^^^^^^\n\n"}
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":2110,"byte_end":2117,"line_start":107,"line_end":107,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        21..=30 => (),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":2133,"byte_end":2140,"line_start":108,"line_end":108,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        21..=40 => (),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:107:9\n   |\nLL |         21..=30 => (),\n   |         ^^^^^^^\n   |\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:108:9\n   |\nLL |         21..=40 => (),\n   |         ^^^^^^^\n\n"}
{"message":"some ranges overlap","code":{"code":"clippy::match_overlapping_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":2364,"byte_end":2389,"line_start":121,"line_end":121,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        0..=0x0000_0000_0000_00ff => (),","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"overlaps with this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_overlapping_arm.rs","byte_start":2405,"byte_end":2430,"line_start":122,"line_end":122,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        0..=0x0000_0000_0000_ffff => (),","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: some ranges overlap\n  --> tests/ui/match_overlapping_arm.rs:121:9\n   |\nLL |         0..=0x0000_0000_0000_00ff => (),\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: overlaps with this\n  --> tests/ui/match_overlapping_arm.rs:122:9\n   |\nLL |         0..=0x0000_0000_0000_ffff => (),\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
