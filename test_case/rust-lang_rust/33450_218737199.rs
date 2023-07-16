
---- [run-make] run-make/dep-info-no-analysis stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
DYLD_LIBRARY_PATH="/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/run-make/dep-info-no-analysis.stage2-x86_64-apple-darwin:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2/lib:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2-test/x86_64-apple-darwin/release/deps:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2-std/x86_64-apple-darwin/release/deps:" '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/run-make/dep-info-no-analysis.stage2-x86_64-apple-darwin -L /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/run-make/dep-info-no-analysis.stage2-x86_64-apple-darwin  -o /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/run-make/dep-info-no-analysis.stage2-x86_64-apple-darwin/input.dd -Z no-analysis --emit dep-info input.rs
sed -i "s/^.*input.dd/input.dd/g" /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/run-make/dep-info-no-analysis.stage2-x86_64-apple-darwin/input.dd

------------------------------------------
stderr:
------------------------------------------
warning: ignoring --out-dir flag due to -o flag.
sed: -i may not be used with stdin
make[1]: *** [all] Error 1

------------------------------------------

thread '[run-make] run-make/dep-info-no-analysis' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/src/tools/compiletest/src/runtest.rs:1543
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-make] run-make/dep-info-no-analysis
