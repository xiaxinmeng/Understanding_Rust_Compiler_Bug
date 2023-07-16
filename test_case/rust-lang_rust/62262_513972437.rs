plain
2019-07-22T20:33:03.0812220Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-22T20:33:03.0812290Z 
2019-07-22T20:33:03.0812537Z   git checkout -b <new-branch-name>
2019-07-22T20:33:03.0812584Z 
2019-07-22T20:33:03.0812876Z HEAD is now at b8ff8c05b Auto merge of #62262 - varkor:must_use-adt-components-ii, r=<try>
2019-07-22T20:33:03.0962914Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-22T20:33:03.1005387Z ==============================================================================
2019-07-22T20:33:03.1005477Z Task         : Bash
2019-07-22T20:33:03.1005563Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-22T21:49:09.5683229Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-22T21:50:04.1368385Z [RUSTC-TIMING] syntax_ext test:false 54.562
2019-07-22T21:55:25.3898916Z [RUSTC-TIMING] rustc test:false 375.813
2019-07-22T21:55:25.3914735Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-22T21:55:44.9931895Z error: unused `std::iter::Map` in field `replace_with` that must be used
2019-07-22T21:55:44.9932325Z    --> src/librustc_mir/transform/add_retag.rs:100:13
2019-07-22T21:55:44.9932637Z     |
2019-07-22T21:55:44.9933041Z 100 | /             basic_blocks[START_BLOCK].statements.splice(0..0,
2019-07-22T21:55:44.9934065Z 101 | |                 places.into_iter().map(|place| Statement {
2019-07-22T21:55:44.9934640Z 102 | |                     source_info,
2019-07-22T21:55:44.9935106Z 103 | |                     kind: StatementKind::Retag(RetagKind::FnEntry, place),
2019-07-22T21:55:44.9935861Z 105 | |             );
2019-07-22T21:55:44.9936160Z     | |______________^
2019-07-22T21:55:44.9936431Z     |
2019-07-22T21:55:44.9936431Z     |
2019-07-22T21:55:44.9936744Z     = note: `#[deny(unused_must_use)]` on by default
2019-07-22T21:55:44.9937141Z     = note: iterators are lazy and do nothing unless consumed
2019-07-22T21:55:45.2061283Z error: aborting due to previous error
2019-07-22T21:55:45.2061425Z 
2019-07-22T21:55:45.3422404Z [RUSTC-TIMING] rustc_mir test:false 19.944
2019-07-22T21:55:45.3422888Z error: Could not compile `rustc_mir`.
2019-07-22T21:55:45.3422888Z error: Could not compile `rustc_mir`.
2019-07-22T21:55:45.3423232Z warning: build failed, waiting for other jobs to finish...
2019-07-22T21:56:46.8915794Z [RUSTC-TIMING] rustc_metadata test:false 81.494
2019-07-22T21:56:46.8975031Z error: build failed
2019-07-22T21:56:46.8991151Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-22T21:56:46.9003952Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-07-22T21:56:46.9004151Z Build completed unsuccessfully in 1:16:05
2019-07-22T21:56:46.9004151Z Build completed unsuccessfully in 1:16:05
2019-07-22T21:56:47.7695782Z ##[error]Bash exited with code '1'.
2019-07-22T21:56:47.7729818Z ##[section]Starting: Upload CPU usage statistics
2019-07-22T21:56:47.7736190Z ==============================================================================
2019-07-22T21:56:47.7736300Z Task         : Bash
2019-07-22T21:56:47.7736372Z Description  : Run a Bash script on macOS, Linux, or Windows
