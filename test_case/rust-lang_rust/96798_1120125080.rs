plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4799baa70d0ff1780ee6dffb743d62c79235ace9 and 4f72081b91eb0a34e8cffdf6e5e780c5057f4ab8
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestS3qT1G/stacked-borrows/2phase.stage-id.stderr
To only update this specific test, also pass `--test-args stacked-borrows/2phase.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestS3qT1G" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestS3qT1G/stacked-borrows/2phase.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-L" "/tmp/compiletestS3qT1G/stacked-borrows/2phase.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.17s
thread 'main' panicked at 'fs::read(stamp) failed with No such file or directory (os error 2) ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp")', src/bootstrap/lib.rs:1375:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:00:01
