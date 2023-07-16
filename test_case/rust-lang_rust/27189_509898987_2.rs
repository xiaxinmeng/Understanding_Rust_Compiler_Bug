
     Running `rustc --edition=2018 --crate-name getuniqid3 /home/user/build/2nonpkgs/rust.stuff/plaincatchup/getuniqid3/src/main.rs --color always --crate-type bin --emit=dep-info,link -C codegen-units=16 -C debuginfo=2 -Z treat-err-as-bug=1 -v -C metadata=6c1f7a137011ccf1 -C extra-filename=-6c1f7a137011ccf1 --out-dir /home/user/build/2nonpkgs/rust.stuff/plaincatchup/target/debug/deps -C incremental=/home/user/build/2nonpkgs/rust.stuff/plaincatchup/target/debug/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/plaincatchup/target/debug/deps -C target-cpu=native`
error: Must use at least one of the features: uuid, rand
  --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/getuniqid3/src/main.rs:11:1
   |
11 | compile_error!("Must use at least one of the features: uuid, rand");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread '<unnamed>' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:547:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-dev (fd7f48b3e 2019-06-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=1 -C codegen-units=16 -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `getuniqid3`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name getuniqid3 /home/user/build/2nonpkgs/rust.stuff/plaincatchup/getuniqid3/src/main.rs --color always --crate-type bin --emit=dep-info,link -C codegen-units=16 -C debuginfo=2 -Z treat-err-as-bug=1 -v -C metadata=6c1f7a137011ccf1 -C extra-filename=-6c1f7a137011ccf1 --out-dir /home/user/build/2nonpkgs/rust.stuff/plaincatchup/target/debug/deps -C incremental=/home/user/build/2nonpkgs/rust.stuff/plaincatchup/target/debug/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/plaincatchup/target/debug/deps -C target-cpu=native` (exit code: 101)
