
% rm -f main && rustc +nightly-2018-12-08 -C opt-level=1 -C lto=fat fat-lto-drop-repro/src/main.rs  && ./main
rm -f main && rustc +nightly-2018-12-08 -C opt-level=1 -C lto=fat fat-lto-drop-repro/src/main.rs  && ./main
About to panic
thread 'main' panicked at '???', src/libcore/option.rs:1008:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Dropping a Droppable
% rm -f main && rustc +nightly-2018-12-14 -C opt-level=1 -C lto=fat fat-lto-drop-repro/src/main.rs  && ./main
rm -f main && rustc +nightly-2018-12-14 -C opt-level=1 -C lto=fat fat-lto-drop-repro/src/main.rs  && ./main
About to panic
thread 'main' panicked at '???', src/libcore/option.rs:1008:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
% rustc +nightly-2018-12-08 --version
rustc +nightly-2018-12-08 --version
rustc 1.32.0-nightly (4a45578bc 2018-12-07)
% rustc +nightly-2018-12-14 --version
rustc +nightly-2018-12-14 --version
rustc 1.32.0-nightly (f4a421ee3 2018-12-13)
% 
