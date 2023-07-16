plain
configure: build.print-step-timings := True
configure: build.metrics        := True
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.download-ci-llvm := if-available
configure: llvm.ccache          := sccache
configure: build.submodules     := False
configure: build.locked-deps    := True
configure: build.cargo-native-static := True
---
   Compiling ignore v0.4.18
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
    Finished dev [unoptimized] target(s) in 58.13s
fatal: unsafe repository ('/checkout' is owned by someone else)
To add an exception for this directory, call:
 git config --global --add safe.directory /checkout
 git config --global --add safe.directory /checkout
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--author=bors@rust-lang.org" "-n1" "--first-parent" "HEAD" "--" "/checkout/src/llvm-project" "/checkout/src/bootstrap/download-ci-llvm-stamp" "/checkout/src/version"
expected success, got: exit status: 128', native.rs:132:20
Build completed unsuccessfully in 0:01:16
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
fatal: unsafe repository ('/checkout' is owned by someone else)
To add an exception for this directory, call:
 git config --global --add safe.directory /checkout
 git config --global --add safe.directory /checkout
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--author=bors@rust-lang.org" "-n1" "--first-parent" "HEAD" "--" "/checkout/src/llvm-project" "/checkout/src/bootstrap/download-ci-llvm-stamp" "/checkout/src/version"
expected success, got: exit status: 128', native.rs:132:20
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
fatal: unsafe repository ('/checkout' is owned by someone else)
To add an exception for this directory, call:
 git config --global --add safe.directory /checkout
 git config --global --add safe.directory /checkout
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--author=bors@rust-lang.org" "-n1" "--first-parent" "HEAD" "--" "/checkout/src/llvm-project" "/checkout/src/bootstrap/download-ci-llvm-stamp" "/checkout/src/version"
expected success, got: exit status: 128', native.rs:132:20
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
fatal: unsafe repository ('/checkout' is owned by someone else)
To add an exception for this directory, call:
 git config --global --add safe.directory /checkout
 git config --global --add safe.directory /checkout
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--author=bors@rust-lang.org" "-n1" "--first-parent" "HEAD" "--" "/checkout/src/llvm-project" "/checkout/src/bootstrap/download-ci-llvm-stamp" "/checkout/src/version"
expected success, got: exit status: 128', native.rs:132:20
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
fatal: unsafe repository ('/checkout' is owned by someone else)
To add an exception for this directory, call:
 git config --global --add safe.directory /checkout
 git config --global --add safe.directory /checkout
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--author=bors@rust-lang.org" "-n1" "--first-parent" "HEAD" "--" "/checkout/src/llvm-project" "/checkout/src/bootstrap/download-ci-llvm-stamp" "/checkout/src/version"
expected success, got: exit status: 128', native.rs:132:20
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
The command has failed after 5 attempts.
