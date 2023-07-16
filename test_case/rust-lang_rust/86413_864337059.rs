plain
+ python2.7 ../x.py --stage 2 test --exclude src/tools/tidy
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
Skipping Set({"src/tools/tidy"}) because it is excluded
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
".git\n"
origin https://github.com/rust-lang/rust (fetch)
origin https://github.com/rust-lang/rust (push)

error: cannot open .git/FETCH_HEAD: Read-only file system
thread 'main' panicked at 'command did not execute successfully: "git" "fetch" "origin" "master"
expected success, got: exit status: 255', src/build_helper/lib.rs:139:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:00
