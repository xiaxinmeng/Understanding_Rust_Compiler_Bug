
$ ./x.py -j1 test
...
rustdoc: [theme-checker] Starting tests!
 - Checking "/tmp/guix-build-rust-1.28.0.drv-0/rustc-1.28.0-src/src/librustdoc/h
tml/static/themes/dark.css"... OK
< RustdocTheme { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu"
 } }
> Bootstrap
running: "/gnu/store/vkh0akdppdj1ddqwprx0czm6i8p158xx-rust-1.27.0-cargo/bin/carg
o" "test" "--" "--test-threads=1"
   Compiling difference v2.0.0
   Compiling ansi_term v0.11.0
   Compiling pretty_assertions v0.5.1
   Compiling bootstrap v0.0.0 (file:///tmp/guix-build-rust-1.28.0.drv-0/rustc-1.
28.0-src/src/bootstrap)
    Finished dev [unoptimized] target(s) in 32.37s
     Running /tmp/guix-build-rust-1.28.0.drv-0/rustc-1.28.0-src/build/bootstrap/
debug/deps/bootstrap-8fbbec03a03ae676

running 9 tests
test builder::__test::build_default ... error: test failed, to rerun pass '--lib
'
command did not execute successfully: "/gnu/store/vkh0akdppdj1ddqwprx0czm6i8p158xx-rust-1.27.0-cargo/bin/cargo" "test" "--" "--test-threads=1"
expected success, got: exit code: 1

Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
...
