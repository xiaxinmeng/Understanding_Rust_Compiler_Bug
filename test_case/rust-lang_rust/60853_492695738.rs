plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:112bd6c4
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:09:07] DirectMap2M:     4120576 kB
[00:09:07] DirectMap1G:    13631488 kB
[00:09:07] + python2.7 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:09:07]     Finished dev [unoptimized] target(s) in 0.39s
[00:09:10] fatal: Not a valid commit name 7158ed9cbea805adf8161d3deaadba2f85b7692e
[00:09:10] thread 'main' panicked at 'command did not execute successfully: "git" "merge-base" "2bafaaf66556ba153fddb00def6d205abb5beff2" "7158ed9cbea805adf8161d3deaadba2f85b7692e"
[00:09:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:09:10] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:09:10] Build completed unsuccessfully in 0:00:02
