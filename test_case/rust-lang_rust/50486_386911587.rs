plain
[00:06:07]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:12]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:12]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:21]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:33] error[E0061]: this function takes 3 parameters but 4 parameters were supplied
[00:06:33]   --> libsyntax/json.rs:59:9
[00:06:33]    |
[00:06:33] 45 | /     pub fn stderr(registry: Option<Registry>,
[00:06:33] 46 | |                   code_map: Lrc<CodeMap>,
[00:06:33] 47 | |                   pretty: bool) -> JsonEmitter {
[00:06:33] 48 | |         JsonEmitter {
[00:06:33] 54 | |         }
[00:06:33] 55 | |     }
[00:06:33]    | |_____- defined here
[00:06:33] ...
[00:06:33] ...
[00:06:33] 59 | /         JsonEmitter::stderr(None, Lrc::new(CodeMap::new(file_path_mapping)),
[00:06:33] 60 | |                             pretty, false)
[00:06:33] 
ff7380f27f57.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-875ad97a99641bc3.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so` (exit code: 101)
ff7380f27f57.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-875ad97a99641bc3.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so` (exit code: 101)
[00:06:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:38] expected success, got: exit code: 101
[00:06:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:38] travis_fold:end:stage0-rustc

[00:06:38] travis_time:end:stage0-rustc:start=1525639165693094394,finish=1525639236235535085,duration=70542440691


[00:06:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:38] Build completed unsuccessfully in 0:01:25
[00:06:38] Makefile:28: recipe for target 'all' failed
[00:06:38] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a3c23f2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
