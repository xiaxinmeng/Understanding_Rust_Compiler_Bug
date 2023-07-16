plain
DirectMap1G:    56623104 kB
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
Skipping Set({"src/tools/tidy"}) because it is excluded
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
"origin\thttps://github.com/rust-lang/rust (fetch)\norigin\thttps://github.com/rust-lang/rust (push)\n"
fatal: ambiguous argument 'refs/remotes/origin/master..HEAD': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--count" "--merges" "refs/remotes/origin/master..HEAD"
expected success, got: exit status: 128', src/build_helper/lib.rs:139:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:00
