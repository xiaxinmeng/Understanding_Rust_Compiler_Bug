plain
[00:01:58]    Compiling serde_json v1.0.15
[00:01:58]    Compiling serde_derive_internals v0.23.1
[00:02:04]    Compiling serde_derive v1.0.40
[00:02:13]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:13] error: expected one of `,`, `.`, `::`, `?`, `}`, or an operator, found `_`
[00:02:13]    --> bootstrap/compile.rs:559:9
[00:02:13]     |
[00:02:13] 558 |         Kind::Test | Kind::Check => builder.config.rust_polly_tests
[00:02:13]     |                                  --                                - expected one of `,`, `.`, `::`, `?`, `}`, or an operator here
[00:02:13]     |                                  |
[00:02:13]     |                                  while parsing the `match` arm starting here
[00:02:13] 559 |         _ => builder.config.rust_polly_self
[00:02:13]     |         ^ unexpected token
[00:02:17] error: unreachable expression
[00:02:17]    --> bootstrap/compile.rs:561:5
[00:02:17]     |
[00:02:17] 561 | /     if use_polly {
[00:02:17] 561 | /     if use_polly {
[00:02:17] 562 | |         cargo.env("RUSTC_USE_POLLY", "1");
[00:02:17] 563 | |     } else {
[00:02:17] 564 | |         cargo.env("RUSTC_USE_POLLY", "0");
[00:02:17]     | |_____^
[00:02:17]     |
[00:02:17] note: lint level defined here
[00:02:17]    --> bootstrap/lib.rs:116:9
[00:02:17]    --> bootstrap/lib.rs:116:9
[00:02:17]     |
[00:02:17] 116 | #![deny(warnings)]
[00:02:17]     |         ^^^^^^^^
[00:02:17]     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
[00:02:19] error: aborting due to 2 previous errors
[00:02:19] 
[00:02:19] 
[00:02:19] error: Could not compile `bootstrap`.
[00:02:19] To learn more, run the command again with --verbose.
[00:02:19] To learn more, run the command again with --verbose.
[00:02:19] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:19] Build completed unsuccessfully in 0:01:33
[00:02:19] make: *** [prepare] Error 1
[00:02:19] Makefile:81: recipe for target 'prepare' failed
[00:02:19]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:19]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:20] error: expected one of `,`, `.`, `::`, `?`, `}`, or an operator, found `_`
[00:02:20]    --> bootstrap/compile.rs:559:9
[00:02:20]     |
[00:02:20] 558 |         Kind::Test | Kind::Check => builder.config.rust_polly_tests
[00:02:20]     |                                  --                                - expected one of `,`, `.`, `::`, `?`, `}`, or an operator here
[00:02:20]     |                                  |
[00:02:20]     |                                  while parsing the `match` arm starting here
[00:02:20] 559 |         _ => builder.config.rust_polly_self
[00:02:20]     |         ^ unexpected token
[00:02:23] error: unreachable expression
[00:02:23]    --> bootstrap/compile.rs:561:5
[00:02:23]     |
[00:02:23] 561 | /     if use_polly {
[00:02:23] 561 | /     if use_polly {
[00:02:23] 562 | |         cargo.env("RUSTC_USE_POLLY", "1");
[00:02:23] 563 | |     } else {
[00:02:23] 564 | |         cargo.env("RUSTC_USE_POLLY", "0");
[00:02:23]     | |_____^
[00:02:23]     |
[00:02:23] note: lint level defined here
[00:02:23]    --> bootstrap/lib.rs:116:9
[00:02:23]    --> bootstrap/lib.rs:116:9
[00:02:23]     |
[00:02:23] 116 | #![deny(warnings)]
[00:02:23]     |         ^^^^^^^^
[00:02:23]     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
[00:02:25] error: aborting due to 2 previous errors
[00:02:25] 
[00:02:25] 
[00:02:25] error: Could not compile `bootstrap`.
[00:02:25] To learn more, run the command again with --verbose.
[00:02:25] To learn more, run the command again with --verbose.
[00:02:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:25] Build completed unsuccessfully in 0:00:06
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:81: recipe for target 'prepare' failed
[00:02:26]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:26]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:26] error: expected one of `,`, `.`, `::`, `?`, `}`, or an operator, found `_`
[00:02:26]    --> bootstrap/compile.rs:559:9
[00:02:26]     |
[00:02:26] 558 |         Kind::Test | Kind::Check => builder.config.rust_polly_tests
[00:02:26]     |                                  --                                - expected one of `,`, `.`, `::`, `?`, `}`, or an operator here
[00:02:26]     |                                  |
[00:02:26]     |                                  while parsing the `match` arm starting here
[00:02:26] 559 |         _ => builder.config.rust_polly_self
[00:02:26]     |         ^ unexpected token
[00:02:29] error: unreachable expression
[00:02:29]    --> bootstrap/compile.rs:561:5
[00:02:29]     |
[00:02:29] 561 | /     if use_polly {
[00:02:29] 561 | /     if use_polly {
[00:02:29] 562 | |         cargo.env("RUSTC_USE_POLLY", "1");
[00:02:29] 563 | |     } else {
[00:02:29] 564 | |         cargo.env("RUSTC_USE_POLLY", "0");
[00:02:29]     | |_____^
[00:02:29]     |
[00:02:29] note: lint level defined here
[00:02:29]    --> bootstrap/lib.rs:116:9
[00:02:29]    --> bootstrap/lib.rs:116:9
[00:02:29]     |
[00:02:29] 116 | #![deny(warnings)]
[00:02:29]     |         ^^^^^^^^
[00:02:29]     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
[00:02:32] error: aborting due to 2 previous errors
[00:02:32] 
[00:02:32] 
[00:02:32] error: Could not compile `bootstrap`.
[00:02:32] To learn more, run the command again with --verbose.
[00:02:32] To learn more, run the command again with --verbose.
[00:02:32] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:32] Build completed unsuccessfully in 0:00:06
[00:02:32] Makefile:81: recipe for target 'prepare' failed
[00:02:32] make: *** [prepare] Error 1
[00:02:32]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:32]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:32] error: expected one of `,`, `.`, `::`, `?`, `}`, or an operator, found `_`
[00:02:32]    --> bootstrap/compile.rs:559:9
[00:02:32]     |
[00:02:32] 558 |         Kind::Test | Kind::Check => builder.config.rust_polly_tests
[00:02:32]     |                                  --                                - expected one of `,`, `.`, `::`, `?`, `}`, or an operator here
[00:02:32]     |                                  |
[00:02:32]     |                                  while parsing the `match` arm starting here
[00:02:32] 559 |         _ => builder.config.rust_polly_self
[00:02:32]     |         ^ unexpected token
[00:02:36] error: unreachable expression
[00:02:36]    --> bootstrap/compile.rs:561:5
[00:02:36]     |
[00:02:36] 561 | /     if use_polly {
[00:02:36] 561 | /     if use_polly {
[00:02:36] 562 | |         cargo.env("RUSTC_USE_POLLY", "1");
[00:02:36] 563 | |     } else {
[00:02:36] 564 | |         cargo.env("RUSTC_USE_POLLY", "0");
[00:02:36]     | |_____^
[00:02:36]     |
[00:02:36] note: lint level defined here
[00:02:36]    --> bootstrap/lib.rs:116:9
[00:02:36]    --> bootstrap/lib.rs:116:9
[00:02:36]     |
[00:02:36] 116 | #![deny(warnings)]
[00:02:36]     |         ^^^^^^^^
[00:02:36]     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
[00:02:38] error: aborting due to 2 previous errors
[00:02:38] 
[00:02:38] 
[00:02:38] error: Could not compile `bootstrap`.
[00:02:38] To learn more, run the command again with --verbose.
[00:02:38] To learn more, run the command again with --verbose.
[00:02:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:38] Build completed unsuccessfully in 0:00:06
[00:02:38] make: *** [prepare] Error 1
[00:02:38] Makefile:81: recipe for target 'prepare' failed
[00:02:39]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:39]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:39] error: expected one of `,`, `.`, `::`, `?`, `}`, or an operator, found `_`
[00:02:39]    --> bootstrap/compile.rs:559:9
[00:02:39]     |
[00:02:39] 558 |         Kind::Test | Kind::Check => builder.config.rust_polly_tests
[00:02:39]     |                                  --                                - expected one of `,`, `.`, `::`, `?`, `}`, or an operator here
[00:02:39]     |                                  |
[00:02:39]     |                                  while parsing the `match` arm starting here
[00:02:39] 559 |         _ => builder.config.rust_polly_self
[00:02:39]     |         ^ unexpected token
[00:02:42] error: unreachable expression
[00:02:42]    --> bootstrap/compile.rs:561:5
[00:02:42]     |
[00:02:42] 561 | /     if use_polly {
[00:02:42] 561 | /     if use_polly {
[00:02:42] 562 | |         cargo.env("RUSTC_USE_POLLY", "1");
[00:02:42] 563 | |     } else {
[00:02:42] 564 | |         cargo.env("RUSTC_USE_POLLY", "0");
[00:02:42]     | |_____^
[00:02:42]     |
[00:02:42] note: lint level defined here
[00:02:42]    --> bootstrap/lib.rs:116:9
[00:02:42]    --> bootstrap/lib.rs:116:9
[00:02:42]     |
[00:02:42] 116 | #![deny(warnings)]
[00:02:42]     |         ^^^^^^^^
[00:02:42]     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
[00:02:45] error: aborting due to 2 previous errors
[00:02:45] 
[00:02:45] 
[00:02:45] error: Could not compile `bootstrap`.
[00:02:45] To learn more, run the command again with --verbose.
[00:02:45] To learn more, run the command again with --verbose.
[00:02:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:45] Build completed unsuccessfully in 0:00:06
[00:02:45] make: *** [prepare] Error 1
[00:02:45] Makefile:81: recipe for target 'prepare' failed
[00:02:45] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1b40bdad
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
