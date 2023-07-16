plain
[00:01:47]    Compiling syn v0.13.1
[00:01:54]    Compiling serde_derive_internals v0.23.1
[00:01:57]    Compiling serde_derive v1.0.40
[00:02:06]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:07] error[E0425]: cannot find value `is_test` in this scope
[00:02:07]    --> bootstrap/compile.rs:519:37
[00:02:07]     |
[00:02:07] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:07] 
[00:02:07] 
[00:02:09] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:09]    --> bootstrap/check.rs:95:9
[00:02:09]     |
[00:02:09] 95  |         rustc_cargo(builder, &mut cargo, true);
[00:02:09]     | 
[00:02:09]     | 
[00:02:09]    ::: bootstrap/compile.rs:515:1
[00:02:09]     |
[00:02:09] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:09]     | ---------------------------------------------------------- defined here
[00:02:09] 
[00:02:09] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:09]    --> bootstrap/check.rs:143:9
[00:02:09]     |
[00:02:09] 143 |         rustc_cargo_env(builder, &mut cargo, true);
[00:02:09]     | 
[00:02:09]     | 
[00:02:09]    ::: bootstrap/compile.rs:522:1
[00:02:09]     |
[00:02:09] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:09]     | -------------------------------------------------------------- defined here
[00:02:09] 
[00:02:09] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:09]     --> bootstrap/test.rs:1512:17
[00:02:09]      |
[00:02:09] 1512 |                 compile::rustc_cargo(builder, &mut cargo, true);
[00:02:09]      | 
[00:02:09]      | 
[00:02:09]     ::: bootstrap/compile.rs:515:1
[00:02:09]      |
[00:02:09] 515  | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:09]      | ---------------------------------------------------------- defined here
[00:02:09] 
[00:02:09] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:09]    --> bootstrap/compile.rs:497:9
[00:02:09]     |
[00:02:09] 497 |         rustc_cargo(builder, &mut cargo, false);
[00:02:09] ...
[00:02:09] ...
[00:02:09] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:09]     | ---------------------------------------------------------- defined here
[00:02:09] 
[00:02:09] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:09]    --> bootstrap/compile.rs:519:5
[00:02:09]     |
[00:02:09] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:09] ...
[00:02:09] ...
[00:02:09] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:09]     | -------------------------------------------------------------- defined here
[00:02:09] 
[00:02:09] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:09]    --> bootstrap/compile.rs:653:9
[00:02:09]     |
[00:02:09] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:09]     | -------------------------------------------------------------- defined here
[00:02:09] ...
[00:02:09] 653 |         rustc_cargo_env(builder, &mut cargo, false);
[00:02:09] 
[00:02:09] 
[00:02:11] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:11]     |
[00:02:11]     |
[00:02:11] 620 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:11]     | 
[00:02:11]     | 
[00:02:11]    ::: bootstrap/compile.rs:515:1
[00:02:11]     |
[00:02:11] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:11]     | ---------------------------------------------------------- defined here
[00:02:11] 
[00:02:11] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:11]     |
[00:02:11]     |
[00:02:11] 696 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:11]     | 
[00:02:11]     | 
[00:02:11]    ::: bootstrap/compile.rs:515:1
[00:02:11]     |
[00:02:11] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:11]     | ---------------------------------------------------------- defined here
[00:02:11] error: aborting due to 9 previous errors
[00:02:11] 
[00:02:11] Some errors occurred: E0061, E0425.
[00:02:11] For more information about an error, try `rustc --explain E0061`.
[00:02:11] For more information about an error, try `rustc --explain E0061`.
[00:02:11] error: Could not compile `bootstrap`.
[00:02:11] To learn more, run the command again with --verbose.
[00:02:11] To learn more, run the command again with --verbose.
[00:02:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:11] Build completed unsuccessfully in 0:01:21
[00:02:11] make: *** [prepare] Error 1
[00:02:11] Makefile:81: recipe for target 'prepare' failed
[00:02:12]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:12]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:13] error[E0425]: cannot find value `is_test` in this scope
[00:02:13]    --> bootstrap/compile.rs:519:37
[00:02:13]     |
[00:02:13] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:13] 
[00:02:13] 
[00:02:15] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:15]    --> bootstrap/check.rs:95:9
[00:02:15]     |
[00:02:15] 95  |         rustc_cargo(builder, &mut cargo, true);
[00:02:15]     | 
[00:02:15]     | 
[00:02:15]    ::: bootstrap/compile.rs:515:1
[00:02:15]     |
[00:02:15] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:15]     | ---------------------------------------------------------- defined here
[00:02:15] 
[00:02:15] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:15]    --> bootstrap/check.rs:143:9
[00:02:15]     |
[00:02:15] 143 |         rustc_cargo_env(builder, &mut cargo, true);
[00:02:15]     | 
[00:02:15]     | 
[00:02:15]    ::: bootstrap/compile.rs:522:1
[00:02:15]     |
[00:02:15] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:15]     | -------------------------------------------------------------- defined here
[00:02:15] 
[00:02:15] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:15]     --> bootstrap/test.rs:1512:17
[00:02:15]      |
[00:02:15] 1512 |                 compile::rustc_cargo(builder, &mut cargo, true);
[00:02:15]      | 
[00:02:15]      | 
[00:02:15]     ::: bootstrap/compile.rs:515:1
[00:02:15]      |
[00:02:15] 515  | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:15]      | ---------------------------------------------------------- defined here
[00:02:15] 
[00:02:15] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:15]    --> bootstrap/compile.rs:497:9
[00:02:15]     |
[00:02:15] 497 |         rustc_cargo(builder, &mut cargo, false);
[00:02:15] ...
[00:02:15] ...
[00:02:15] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:15]     | ---------------------------------------------------------- defined here
[00:02:15] 
[00:02:15] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:15]    --> bootstrap/compile.rs:519:5
[00:02:15]     |
[00:02:15] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:15] ...
[00:02:15] ...
[00:02:15] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:15]     | -------------------------------------------------------------- defined here
[00:02:15] 
[00:02:15] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:15]    --> bootstrap/compile.rs:653:9
[00:02:15]     |
[00:02:15] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:15]     | -------------------------------------------------------------- defined here
[00:02:15] ...
[00:02:15] 653 |         rustc_cargo_env(builder, &mut cargo, false);
[00:02:15] 
[00:02:15] 
[00:02:17] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:17]     |
[00:02:17]     |
[00:02:17] 620 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:17]     | 
[00:02:17]     | 
[00:02:17]    ::: bootstrap/compile.rs:515:1
[00:02:17]     |
[00:02:17] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:17]     | ---------------------------------------------------------- defined here
[00:02:17] 
[00:02:17] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:17]     |
[00:02:17]     |
[00:02:17] 696 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:17]     | 
[00:02:17]     | 
[00:02:17]    ::: bootstrap/compile.rs:515:1
[00:02:17]     |
[00:02:17] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:17]     | ---------------------------------------------------------- defined here
[00:02:17] error: aborting due to 9 previous errors
[00:02:17] 
[00:02:17] Some errors occurred: E0061, E0425.
[00:02:17] For more information about an error, try `rustc --explain E0061`.
[00:02:17] For more information about an error, try `rustc --explain E0061`.
[00:02:17] error: Could not compile `bootstrap`.
[00:02:17] To learn more, run the command again with --verbose.
[00:02:17] To learn more, run the command again with --verbose.
[00:02:17] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:17] Build completed unsuccessfully in 0:00:06
[00:02:17] Makefile:81: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:18]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:18]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:20] error[E0425]: cannot find value `is_test` in this scope
[00:02:20]    --> bootstrap/compile.rs:519:37
[00:02:20]     |
[00:02:20] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:20] 
[00:02:20] 
[00:02:21] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:21]    --> bootstrap/check.rs:95:9
[00:02:21]     |
[00:02:21] 95  |         rustc_cargo(builder, &mut cargo, true);
[00:02:21]     | 
[00:02:21]     | 
[00:02:21]    ::: bootstrap/compile.rs:515:1
[00:02:21]     |
[00:02:21] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:21]     | ---------------------------------------------------------- defined here
[00:02:21] 
[00:02:21] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:21]    --> bootstrap/check.rs:143:9
[00:02:21]     |
[00:02:21] 143 |         rustc_cargo_env(builder, &mut cargo, true);
[00:02:21]     | 
[00:02:21]     | 
[00:02:21]    ::: bootstrap/compile.rs:522:1
[00:02:21]     |
[00:02:21] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:21]     | -------------------------------------------------------------- defined here
[00:02:21] 
[00:02:21] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:21]     --> bootstrap/test.rs:1512:17
[00:02:21]      |
[00:02:21] 1512 |                 compile::rustc_cargo(builder, &mut cargo, true);
[00:02:21]      | 
[00:02:21]      | 
[00:02:21]     ::: bootstrap/compile.rs:515:1
[00:02:21]      |
[00:02:21] 515  | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:21]      | ---------------------------------------------------------- defined here
[00:02:21] 
[00:02:22] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:22]    --> bootstrap/compile.rs:497:9
[00:02:22]     |
[00:02:22] 497 |         rustc_cargo(builder, &mut cargo, false);
[00:02:22] ...
[00:02:22] ...
[00:02:22] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:22]     | ---------------------------------------------------------- defined here
[00:02:22] 
[00:02:22] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:22]    --> bootstrap/compile.rs:519:5
[00:02:22]     |
[00:02:22] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:22] ...
[00:02:22] ...
[00:02:22] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:22]     | -------------------------------------------------------------- defined here
[00:02:22] 
[00:02:22] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:22]    --> bootstrap/compile.rs:653:9
[00:02:22]     |
[00:02:22] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:22]     | -------------------------------------------------------------- defined here
[00:02:22] ...
[00:02:22] 653 |         rustc_cargo_env(builder, &mut cargo, false);
[00:02:22] 
[00:02:22] 
[00:02:23] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:23]     |
[00:02:23]     |
[00:02:23] 620 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:23]     | 
[00:02:23]     | 
[00:02:23]    ::: bootstrap/compile.rs:515:1
[00:02:23]     |
[00:02:23] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:23]     | ---------------------------------------------------------- defined here
[00:02:23] 
[00:02:23] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:23]     |
[00:02:23]     |
[00:02:23] 696 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:23]     | 
[00:02:23]     | 
[00:02:23]    ::: bootstrap/compile.rs:515:1
[00:02:23]     |
[00:02:23] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:23]     | ---------------------------------------------------------- defined here
[00:02:24] error: aborting due to 9 previous errors
[00:02:24] 
[00:02:24] Some errors occurred: E0061, E0425.
[00:02:24] For more information about an error, try `rustc --explain E0061`.
[00:02:24] For more information about an error, try `rustc --explain E0061`.
[00:02:24] error: Could not compile `bootstrap`.
[00:02:24] To learn more, run the command again with --verbose.
[00:02:24] To learn more, run the command again with --verbose.
[00:02:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:24] Build completed unsuccessfully in 0:00:06
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:24] make: *** [prepare] Error 1
[00:02:24]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:24]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:26] error[E0425]: cannot find value `is_test` in this scope
[00:02:26]    --> bootstrap/compile.rs:519:37
[00:02:26]     |
[00:02:26] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:26] 
[00:02:26] 
[00:02:27] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:27]    --> bootstrap/check.rs:95:9
[00:02:27]     |
[00:02:27] 95  |         rustc_cargo(builder, &mut cargo, true);
[00:02:27]     | 
[00:02:27]     | 
[00:02:27]    ::: bootstrap/compile.rs:515:1
[00:02:27]     |
[00:02:27] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:27]     | ---------------------------------------------------------- defined here
[00:02:27] 
[00:02:27] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:27]    --> bootstrap/check.rs:143:9
[00:02:27]     |
[00:02:27] 143 |         rustc_cargo_env(builder, &mut cargo, true);
[00:02:27]     | 
[00:02:27]     | 
[00:02:27]    ::: bootstrap/compile.rs:522:1
[00:02:27]     |
[00:02:27] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:27]     | -------------------------------------------------------------- defined here
[00:02:27] 
[00:02:28] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:28]     --> bootstrap/test.rs:1512:17
[00:02:28]      |
[00:02:28] 1512 |                 compile::rustc_cargo(builder, &mut cargo, true);
[00:02:28]      | 
[00:02:28]      | 
[00:02:28]     ::: bootstrap/compile.rs:515:1
[00:02:28]      |
[00:02:28] 515  | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:28]      | ---------------------------------------------------------- defined here
[00:02:28] 
[00:02:28] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:28]    --> bootstrap/compile.rs:497:9
[00:02:28]     |
[00:02:28] 497 |         rustc_cargo(builder, &mut cargo, false);
[00:02:28] ...
[00:02:28] ...
[00:02:28] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:28]     | ---------------------------------------------------------- defined here
[00:02:28] 
[00:02:28] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:28]    --> bootstrap/compile.rs:519:5
[00:02:28]     |
[00:02:28] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:28] ...
[00:02:28] ...
[00:02:28] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:28]     | -------------------------------------------------------------- defined here
[00:02:28] 
[00:02:28] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:28]    --> bootstrap/compile.rs:653:9
[00:02:28]     |
[00:02:28] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:28]     | -------------------------------------------------------------- defined here
[00:02:28] ...
[00:02:28] 653 |         rustc_cargo_env(builder, &mut cargo, false);
[00:02:28] 
[00:02:28] 
[00:02:29] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:29]     |
[00:02:29]     |
[00:02:29] 620 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:29]     | 
[00:02:29]     | 
[00:02:29]    ::: bootstrap/compile.rs:515:1
[00:02:29]     |
[00:02:29] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:29]     | ---------------------------------------------------------- defined here
[00:02:29] 
[00:02:29] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:29]     |
[00:02:29]     |
[00:02:29] 696 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:29]     | 
[00:02:29]     | 
[00:02:29]    ::: bootstrap/compile.rs:515:1
[00:02:29]     |
[00:02:29] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:29]     | ---------------------------------------------------------- defined here
[00:02:30] error: aborting due to 9 previous errors
[00:02:30] 
[00:02:30] Some errors occurred: E0061, E0425.
[00:02:30] For more information about an error, try `rustc --explain E0061`.
[00:02:30] For more information about an error, try `rustc --explain E0061`.
[00:02:30] error: Could not compile `bootstrap`.
[00:02:30] To learn more, run the command again with --verbose.
[00:02:30] To learn more, run the command again with --verbose.
[00:02:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:30] Build completed unsuccessfully in 0:00:06
[00:02:30] Makefile:81: recipe for target 'prepare' failed
[00:02:30] make: *** [prepare] Error 1
[00:02:31]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:31]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:32] error[E0425]: cannot find value `is_test` in this scope
[00:02:32]    --> bootstrap/compile.rs:519:37
[00:02:32]     |
[00:02:32] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:32] 
[00:02:32] 
[00:02:33] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:33]    --> bootstrap/check.rs:95:9
[00:02:33]     |
[00:02:33] 95  |         rustc_cargo(builder, &mut cargo, true);
[00:02:33]     | 
[00:02:33]     | 
[00:02:33]    ::: bootstrap/compile.rs:515:1
[00:02:33]     |
[00:02:33] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:33]     | ---------------------------------------------------------- defined here
[00:02:33] 
[00:02:33] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:33]    --> bootstrap/check.rs:143:9
[00:02:33]     |
[00:02:33] 143 |         rustc_cargo_env(builder, &mut cargo, true);
[00:02:33]     | 
[00:02:33]     | 
[00:02:33]    ::: bootstrap/compile.rs:522:1
[00:02:33]     |
[00:02:33] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:33]     | -------------------------------------------------------------- defined here
[00:02:33] 
[00:02:34] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:34]     --> bootstrap/test.rs:1512:17
[00:02:34]      |
[00:02:34] 1512 |                 compile::rustc_cargo(builder, &mut cargo, true);
[00:02:34]      | 
[00:02:34]      | 
[00:02:34]     ::: bootstrap/compile.rs:515:1
[00:02:34]      |
[00:02:34] 515  | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:34]      | ---------------------------------------------------------- defined here
[00:02:34] 
[00:02:34] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:34]    --> bootstrap/compile.rs:497:9
[00:02:34]     |
[00:02:34] 497 |         rustc_cargo(builder, &mut cargo, false);
[00:02:34] ...
[00:02:34] ...
[00:02:34] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:34]     | ---------------------------------------------------------- defined here
[00:02:34] 
[00:02:34] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:34]    --> bootstrap/compile.rs:519:5
[00:02:34]     |
[00:02:34] 519 |     rustc_cargo_env(builder, cargo, is_test);
[00:02:34] ...
[00:02:34] ...
[00:02:34] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:34]     | -------------------------------------------------------------- defined here
[00:02:34] 
[00:02:34] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:34]    --> bootstrap/compile.rs:653:9
[00:02:34]     |
[00:02:34] 522 | pub fn rustc_cargo_env(builder: &Builder, cargo: &mut Command) {
[00:02:34]     | -------------------------------------------------------------- defined here
[00:02:34] ...
[00:02:34] 653 |         rustc_cargo_env(builder, &mut cargo, false);
[00:02:34] 
[00:02:34] 
[00:02:35] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:35]     |
[00:02:35]     |
[00:02:35] 620 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:35]     | 
[00:02:35]     | 
[00:02:35]    ::: bootstrap/compile.rs:515:1
[00:02:35]     |
[00:02:35] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:35]     | ---------------------------------------------------------- defined here
[00:02:35] 
[00:02:35] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:02:35]     |
[00:02:35]     |
[00:02:35] 696 |         compile::rustc_cargo(builder, &mut cargo, true);
[00:02:35]     | 
[00:02:35]     | 
[00:02:35]    ::: bootstrap/compile.rs:515:1
[00:02:35]     |
[00:02:35] 515 | pub fn rustc_cargo(builder: &Builder, cargo: &mut Command) {
[00:02:35]     | ---------------------------------------------------------- defined here
[00:02:36] error: aborting due to 9 previous errors
[00:02:36] 
[00:02:36] Some errors occurred: E0061, E0425.
[00:02:36] For more information about an error, try `rustc --explain E0061`.
[00:02:36] For more information about an error, try `rustc --explain E0061`.
