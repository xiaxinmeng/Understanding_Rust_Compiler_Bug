plain
2019-07-22T21:12:27.4546506Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-22T21:12:27.4546554Z 
2019-07-22T21:12:27.4546791Z   git checkout -b <new-branch-name>
2019-07-22T21:12:27.4546832Z 
2019-07-22T21:12:27.4547115Z HEAD is now at acb4fefbf Auto merge of #62262 - varkor:must_use-adt-components-ii, r=<try>
2019-07-22T21:12:27.4714486Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-22T21:12:27.4717882Z ==============================================================================
2019-07-22T21:12:27.4717956Z Task         : Bash
2019-07-22T21:12:27.4718028Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-22T22:36:23.4106649Z [RUSTC-TIMING] rustc_allocator test:false 20.334
2019-07-22T22:36:23.4143906Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-22T22:38:46.4423790Z [RUSTC-TIMING] rustc_metadata test:false 163.368
2019-07-22T22:38:46.4428413Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-22T22:39:05.0295112Z error: unused `std::iter::Map` in field `replace_with` that must be used
2019-07-22T22:39:05.0303003Z    --> src/librustc_mir/transform/add_retag.rs:100:13
2019-07-22T22:39:05.0303579Z     |
2019-07-22T22:39:05.0304102Z 100 | /             basic_blocks[START_BLOCK].statements.splice(0..0,
2019-07-22T22:39:05.0304933Z 101 | |                 places.into_iter().map(|place| Statement {
2019-07-22T22:39:05.0305482Z 102 | |                     source_info,
2019-07-22T22:39:05.0306570Z 103 | |                     kind: StatementKind::Retag(RetagKind::FnEntry, place),
2019-07-22T22:39:05.0307802Z 105 | |             );
2019-07-22T22:39:05.0308302Z     | |______________^
2019-07-22T22:39:05.0308754Z     |
2019-07-22T22:39:05.0308754Z     |
2019-07-22T22:39:05.0309246Z     = note: `#[deny(unused_must_use)]` on by default
2019-07-22T22:39:05.0310095Z     = note: iterators are lazy and do nothing unless consumed
2019-07-22T22:39:05.0311006Z error: aborting due to previous error
2019-07-22T22:39:05.0311205Z 
2019-07-22T22:39:05.0311571Z [RUSTC-TIMING] rustc_mir test:false 18.374
2019-07-22T22:39:05.0311939Z error: Could not compile `rustc_mir`.
2019-07-22T22:39:05.0311939Z error: Could not compile `rustc_mir`.
2019-07-22T22:39:05.0312356Z warning: build failed, waiting for other jobs to finish...
2019-07-22T22:39:45.2721880Z [RUSTC-TIMING] rustc_typeck test:false 201.852
2019-07-22T22:39:45.2803518Z error: build failed
2019-07-22T22:39:45.2832785Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-22T22:39:45.2842826Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-07-22T22:39:45.2843226Z Build completed unsuccessfully in 1:19:53
2019-07-22T22:39:45.2843226Z Build completed unsuccessfully in 1:19:53
2019-07-22T22:39:46.3986438Z ##[error]Bash exited with code '1'.
2019-07-22T22:39:46.4062348Z ##[section]Starting: Upload CPU usage statistics
2019-07-22T22:39:46.4072442Z ==============================================================================
2019-07-22T22:39:46.4072535Z Task         : Bash
2019-07-22T22:39:46.4072621Z Description  : Run a Bash script on macOS, Linux, or Windows
