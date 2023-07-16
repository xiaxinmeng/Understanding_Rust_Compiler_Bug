
$ nice $SRC_DIR/x.py build --stage 1 src/rustc --host=$HOST
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/liblibc'
Synchronizing submodule url for 'src/llvm'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
HEAD is now at a8fc4c1 Merge pull request #28 from xen0n/preprocessor-firefighting
HEAD is now at e058ca6 Change how the default zone is found
HEAD is now at 6e8c1b4 Merge pull request #446 from alexcrichton/link-cfg
HEAD is now at c1d9622 Backport r285278 [ARM] Predicate UMAAL selection on hasDSP.
HEAD is now at a3736a0 Merge pull request #6 from intelfx/patch-1
HEAD is now at 4f99485 Merge pull request #54 from brson/docdir
Build completed in 0:00:02

$ tree -d build/
build/
|-- bootstrap
|   `-- debug
|       |-- build
|       |-- deps
|       |-- examples
|       `-- native
|-- cache
|   |-- 2016-09-26
|   `-- 2016-09-28
`-- x86_64-unknown-linux-gnu
    `-- stage0
        |-- bin
        |-- etc
        |   `-- bash_completion.d
        |-- lib
        |   |-- cargo
        |   `-- rustlib
        |       |-- etc
        |       `-- x86_64-unknown-linux-gnu
        |           |-- bin
        |           `-- lib
        `-- share
            |-- doc
            |   |-- cargo
            |   `-- rust
            |-- man
            |   `-- man1
            `-- zsh
                `-- site-functions

29 directories
