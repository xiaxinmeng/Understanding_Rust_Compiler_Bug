plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]   --> tools/tidy/src/lib.rs:28:74
[00:05:04]    |
[00:05:04] 28 |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]    |                                                                          ^^^^^^^
[00:05:04]    |
[00:05:04]   ::: tools/tidy/src/cargo.rs:26:18
[00:05:04]    |
[00:05:04] 26 |     for entry in t!(path.read_dir(), path).map(|e| t!(e)) {
[00:05:04]    |                  ------------------------- in this macro invocation
[00:05:04]    |
[00:05:04]    = note: `-D deprecated` implied by `-D warnings`
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]   --> tools/tidy/src/lib.rs:28:74
[00:05:04]    |
[00:05:04] 28 |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]    |                                                                          ^^^^^^^
[00:05:04] ...
[00:05:04] 90 |     for entry in t!(fs::read_dir(path), path) {
[00:05:04]    |                  ---------------------------- in this macro invocation
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]   --> tools/tidy/src/lib.rs:28:74
[00:05:04]    |
[00:05:04] 28 |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]    |                                                                          ^^^^^^^
[00:05:04]    |
[00:05:04]   ::: tools/tidy/src/bins.rs:50:24
[00:05:04]    |
[00:05:04] 50 |         let metadata = t!(fs::symlink_metadata(&file), &file);
[00:05:04]    |                        -------------------------------------- in this macro invocation
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]   --> tools/tidy/src/bins.rs:65:73
[00:05:04]    |
[00:05:04] 65 |                 tidy_error!(bad, "binary checked into source: {}", file.display());
[00:05:04]    |                                                                         ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/lib.rs:28:74
[00:05:04]     |
[00:05:04] 28  |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]     |                                                                          ^^^^^^^
[00:05:04]     |
[00:05:04]    ::: tools/tidy/src/style.rs:123:12
[00:05:04]     |
[00:05:04] 123 |         t!(t!(File::open(file), file).read_to_string(&mut contents));
[00:05:04]     |            -------------------------- in this macro invocation
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/style.rs:126:53
[00:05:04]     |
[00:05:04] 126 |             tidy_error!(bad, "{}: empty file", file.display());
[00:05:04]     |                                                     ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/style.rs:136:52
[00:05:04]     |
[00:05:04] 136 |                 tidy_error!(bad, "{}:{}: {}", file.display(), i + 1, msg);
[00:05:04]     |                                                    ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/style.rs:172:60
[00:05:04]     |
[00:05:04] 172 |             tidy_error!(bad, "{}: incorrect license", file.display());
[00:05:04]     |                                                            ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/style.rs:175:72
[00:05:04]     |
[00:05:04] 175 |             0 => tidy_error!(bad, "{}: missing trailing newline", file.display()),
[00:05:04]     |                                                                        ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/style.rs:177:79
[00:05:04]     |
[00:05:04] 177 |             n => tidy_error!(bad, "{}: too many trailing newlines ({})", file.display(), n),
[00:05:04]     |                                                                               ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]   --> tools/tidy/src/errors.rs:84:48
[00:05:04]    |
[00:05:04] 84 |             tidy_error!(bad, "{}:{}: {}", file.display(), line_num, line);
[00:05:04]    |                                                ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/lib.rs:28:74
[00:05:04]     |
[00:05:04] 28  |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]     |                                                                          ^^^^^^^
[00:05:04]     |
[00:05:04]    ::: tools/tidy/src/features.rs:110:12
[00:05:04]     |
[00:05:04] 110 |         t!(t!(File::open(&file), &file).read_to_string(&mut contents));
[00:05:04]     |            ---------------------------- in this macro invocation
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/features.rs:114:52
[00:05:04]     |
[00:05:04] 114 |                 tidy_error!(bad, "{}:{}: {}", file.display(), i + 1, msg);
[00:05:04]     |                                                    ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/features.rs:293:50
[00:05:04]     |
[00:05:04] 293 |                                             file.display(),
[00:05:04]     |                                                  ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/features.rs:305:56
[00:05:04]     |
[00:05:04] 305 |                     tidy_error!(bad, "{}:{}: {}", file.display(), line, msg);
[00:05:04]     |                                                        ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/lib.rs:28:74
[00:05:04]     |
[00:05:04] 28  |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]     |                                                                          ^^^^^^^
[00:05:04]     |
[00:05:04]    ::: tools/tidy/src/features.rs:326:12
[00:05:04]     |
[00:05:04] 326 |         t!(t!(File::open(&file), &file).read_to_string(&mut contents));
[00:05:04]     |            ---------------------------- in this macro invocation
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]   --> tools/tidy/src/cargo.rs:96:55
[00:05:04]    |
[00:05:04] 96 |                               depends on it", libfile.display(), krate);
[00:05:04]    |                                                       ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/lib.rs:28:74
[00:05:04]     |
[00:05:04] 28  |         Err(e) => panic!("{} failed on {} with {}", stringify!($e), ($p).display(), e),
[00:05:04]     |                                                                          ^^^^^^^
[00:05:04]     |
[00:05:04]    ::: tools/tidy/src/pal.rs:112:8
[00:05:04]     |
[00:05:04] 112 |     t!(t!(File::open(file), file).read_to_string(contents));
[00:05:04]     |        -------------------------- in this macro invocation
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/pal.rs:130:67
[00:05:04]     |
[00:05:04] 130 |         tidy_error!(bad, "{}:{}: platform-specific cfg: {}", file.display(), line, cfg);
[00:05:04]     |                                                                   ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/deps.rs:229:42
[00:05:04]     |
[00:05:04] 229 |         panic!("{} does not exist", path.display());
[00:05:04]     |                                          ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/deps.rs:241:64
[00:05:04]     |
[00:05:04] 241 |             println!("invalid license {} in {}", license, path.display());
[00:05:04]     |                                                                ^^^^^^^
[00:05:04]
[00:05:04] error: use of deprecated item 'std::path::Path::display': Path and PathBuf implement Display directly
[00:05:04]    --> tools/tidy/src/deps.rs:248:43
[00:05:04]     |
[00:05:04] 248 |         println!("no license in {}", path.display());
[00:05:04]     |                                           ^^^^^^^
[00:05:04]
[00:05:04] error: aborting due to 22 previous errors
[00:05:04]
[00:05:04] error: Could not compile `tidy`.
[00:05:04]
[00:05:04] Caused by:
[00:05:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name tidy tools/tidy/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=ec0ed652cc88252d -C extra-filename=-ec0ed652cc88252d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps/libserde_derive-6fd95ab87191bb85.so --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-b3e872b56e3c8f47.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde-92847b096fc60ebd.rlib` (exit code: 101)
[00:05:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--features" "" "--message-format" "json"
[00:05:04] expected success, got: exit code: 101
[00:05:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:04] Build completed unsuccessfully in 0:01:50
[00:05:04] make: *** [tidy] Error 1
[00:05:04] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:03b47ecb:start=1523441936035098256,finish=1523441936041781453,duration=6683197
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1c787166
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1c787166:start=1523441936047411082,finish=1523441936053841006,duration=6429924
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03634257
$ dmesg | grep -i kill
[   11.039019] init: failsafe main process (1092) killed by TERM signal
