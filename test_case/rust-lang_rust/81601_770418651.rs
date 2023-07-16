plain
+ python2.7 ../x.py --stage 2 test --exclude src/tools/tidy
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
Skipping Set({"src/tools/tidy"}) because it is excluded
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
fatal: invalid gitfile format: /checkout/src/llvm-project/.git
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:00
