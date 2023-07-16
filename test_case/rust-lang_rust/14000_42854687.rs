
% pwd
/home/pnkfelix/Dev/Mozilla/rust-stage1/src/test/run-make/bootstrap-from-c-with-green
% /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/stage2/bin/rustc --out-dir /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green -L /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green lib.rs
% ln -nsf /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot-*.so /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so
% gcc -Wall -Werror -g -fPIC -m64 -L /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green main.c -o /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/main -lboot
/usr/bin/ld: warning: libgreen-f3eb09bc-0.11-pre.so, needed by /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so, not found (try using -rpath or -rpath-link)
/usr/bin/ld: warning: librustuv-30cdf6fe-0.11-pre.so, needed by /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so, not found (try using -rpath or -rpath-link)
/usr/bin/ld: warning: libstd-aad93cea-0.11-pre.so, needed by /home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so, not found (try using -rpath or -rpath-link)
/home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so: undefined reference to `io::stdio::println_args::h8c5752c79c70a99fhjq::v0.11.pre'
/home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so: undefined reference to `start::h24799cd62201bf6aMyc::v0.11.pre'
/home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so: undefined reference to `event_loop::he8e2d0b4e6bf62fey9f::v0.11.pre'
/home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so: undefined reference to `rust_stack_exhausted'
/home/pnkfelix/Dev/Mozilla/rust-stage1/objdir-dbgopt-check/x86_64-unknown-linux-gnu/test/run-make/bootstrap-from-c-with-green/libboot.so: undefined reference to `task::spawn::hb9c9406bc3ba0499Nbh::v0.11.pre'
collect2: error: ld returned 1 exit status
