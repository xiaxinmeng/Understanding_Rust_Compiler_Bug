
fatal: Not a valid commit name b2e36e6c2d229126b59e892c9147fbb68115d292
thread 'main' panicked at 'command did not execute successfully: "git" "merge-base" "a7d891e31a85269d0d0164258f2b6b6670129763" "b2e36e6c2d229126b59e892c9147fbb68115d292"
expected success, got: exit code: 128', src/build_helper/lib.rs:131:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
