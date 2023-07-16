
〉master:rust〉git switch -c new-branch
Switched to a new branch 'new-branch'
〉new-branch:rust〉python ./x.py setup
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.07s
Welcome to the Rust project! What do you want to do with x.py?
a) library: Contribute to the standard library
b) compiler: Contribute to the compiler itself
c) codegen: Contribute to the compiler, and also modify LLVM or codegen
d) tools: Contribute to tools which depend on the compiler, but do not modify it directly (e.g. rustdoc, clippy, miri)
e) user: Install Rust from source
Please choose one (a/b/c/d/e): b
Updating submodule src/tools/cargo
remote: Total 0 (delta 0), reused 0 (delta 0), pack-reused 0
remote: Enumerating objects: 47, done.
remote: Counting objects: 100% (47/47), done.
remote: Compressing objects: 100% (23/23), done.
remote: Total 24 (delta 19), reused 5 (delta 0), pack-reused 0
Unpacking objects: 100% (24/24), 2.77 KiB | 78.00 KiB/s, done.
From https://github.com/rust-lang/cargo
 * branch            6da726708a4406f31f996d813790818dce837161 -> FETCH_HEAD
Submodule path 'src/tools/cargo': checked out '6da726708a4406f31f996d813790818dce837161'
Updating submodule src/tools/miri
Submodule 'src/tools/miri' (https://github.com/rust-lang/miri.git) registered for path 'src/tools/miri'
Cloning into 'D:/tmp/rust/src/tools/miri'...
remote: Enumerating objects: 1278, done.
remote: Counting objects: 100% (1278/1278), done.
remote: Compressing objects: 100% (893/893), done.
remote: Total 1278 (delta 455), reused 653 (delta 332), pack-reused 0
Receiving objects: 100% (1278/1278), 608.55 KiB | 4.95 MiB/s, done.
Resolving deltas: 100% (455/455), done.
remote: Total 0 (delta 0), reused 0 (delta 0), pack-reused 0
remote: Enumerating objects: 179, done.
remote: Counting objects: 100% (179/179), done.
remote: Compressing objects: 100% (90/90), done.
remote: Total 106 (delta 69), reused 27 (delta 13), pack-reused 0Receiving objects:  95% (101/106)
Receiving objects: 100% (106/106), 69.45 KiB | 13.89 MiB/s, done.
Resolving deltas: 100% (69/69), completed with 46 local objects.
From https://github.com/rust-lang/miri
 * branch            ab88e64b152d3704c35db96dbbc6efaaed67773f -> FETCH_HEAD
Submodule path 'src/tools/miri': checked out 'ab88e64b152d3704c35db96dbbc6efaaed67773f'
Updating submodule library/backtrace
Submodule 'library/backtrace' (https://github.com/rust-lang/backtrace-rs.git) registered for path 'library/backtrace'
Cloning into 'D:/tmp/rust/library/backtrace'...
remote: Enumerating objects: 136, done.
remote: Counting objects: 100% (136/136), done.
remote: Compressing objects: 100% (102/102), done.
remote: Total 136 (delta 9), reused 75 (delta 4), pack-reused 0
Receiving objects: 100% (136/136), 93.77 KiB | 2.47 MiB/s, done.
Resolving deltas: 100% (9/9), done.
remote: Total 0 (delta 0), reused 0 (delta 0), pack-reused 0
remote: Enumerating objects: 5, done.
remote: Counting objects: 100% (5/5), done.
remote: Compressing objects: 100% (3/3), done.
remote: Total 3 (delta 2), reused 1 (delta 0), pack-reused 0
Unpacking objects: 100% (3/3), 706 bytes | 117.00 KiB/s, done.
From https://github.com/rust-lang/backtrace-rs
 * branch            4e5a3f72929f152752d5659e95bb15c8f6b41eff -> FETCH_HEAD
Submodule path 'library/backtrace': checked out '4e5a3f72929f152752d5659e95bb15c8f6b41eff'
Updating submodule library/stdarch
Submodule 'library/stdarch' (https://github.com/rust-lang/stdarch.git) registered for path 'library/stdarch'
Cloning into 'D:/tmp/rust/library/stdarch'...
remote: Enumerating objects: 317, done.
remote: Counting objects: 100% (317/317), done.
remote: Compressing objects: 100% (268/268), done.
remote: Total 317 (delta 50), reused 110 (delta 12), pack-reused 0
Receiving objects: 100% (317/317), 1.39 MiB | 4.14 MiB/s, done.
Resolving deltas: 100% (50/50), done.
Submodule path 'library/stdarch': checked out '42df7394d38bc7b945116ea3ad8a7cbcd1db50a9'
Submodule 'crates/intrinsic-test/acle' (https://github.com/ARM-software/acle.git) registered for path 'library/stdarch/crates/intrinsic-test/acle'
Cloning into 'D:/tmp/rust/library/stdarch/crates/intrinsic-test/acle'...
remote: Enumerating objects: 87, done.
remote: Counting objects: 100% (87/87), done.
remote: Compressing objects: 100% (83/83), done.
remote: Total 87 (delta 18), reused 31 (delta 1), pack-reused 0
Receiving objects: 100% (87/87), 616.53 KiB | 4.28 MiB/s, done.
Resolving deltas: 100% (18/18), done.
remote: Total 0 (delta 0), reused 0 (delta 0), pack-reused 0
remote: Enumerating objects: 67, done.
remote: Counting objects: 100% (67/67), done.
remote: Compressing objects: 100% (38/38), done.
remote: Total 40 (delta 25), reused 5 (delta 0), pack-reused 0
Unpacking objects: 100% (40/40), 255.02 KiB | 1008.00 KiB/s, done.
From https://github.com/ARM-software/acle
 * branch            5626f85f469f419db16f20b1614863aeb377c22b -> FETCH_HEAD
Submodule path 'library/stdarch/crates/intrinsic-test/acle': checked out '5626f85f469f419db16f20b1614863aeb377c22b'
`x.py` will now use the configuration at D:\tmp\rust/src/bootstrap/defaults/config.compiler.toml

`stage1` toolchain already linked; not attempting to link `stage1` toolchain

Rust's CI will automatically fail if it doesn't pass `tidy`, the internal tool for ensuring code quality.
If you'd like, x.py can install a git hook for you that will automatically run `tidy --bless` before
pushing your code to ensure your code is up to par. If you decide later that this behavior is
undesirable, simply delete the `pre-push` file from .git/hooks.
Would you like to install the git hook?: [y/N]
Ok, skipping installation!

To get started, try one of the following commands:
- `x.py check`
- `x.py build`
- `x.py test`
For more suggestions, see https://rustc-dev-guide.rust-lang.org/building/suggested.html
Build completed successfully in 0:00:17
