plain
+ python2.7 ../x.py --stage 2 test --exclude src/tools/tidy
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
Skipping Set({"src/tools/tidy"}) because it is excluded
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
".git\n"
"origin\thttps://github.com/rust-lang/rust (fetch)\norigin\thttps://github.com/rust-lang/rust (push)\n"
"total 12\ndrwxr-xr-x 3 user 116 4096 Jun 19 01:17 pull\ndrwxr-xr-x 5 user 116 4096 Jun 19 01:17 ..\ndrwxr-xr-x 3 user 116 4096 Jun 19 01:17 .\n"
ls: cannot access '.git/refs/remotes/origin': No such file or directory
thread 'main' panicked at 'command did not execute successfully: "ls" "-trla" ".git/refs/remotes/origin"
expected success, got: exit status: 2', src/build_helper/lib.rs:139:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:00
