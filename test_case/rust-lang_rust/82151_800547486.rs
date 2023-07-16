
% rustc +stable --version
rustc 1.50.0 (cb75ad5db 2021-02-10)
% rustc +beta --version
rustc 1.51.0-beta.6 (6a1835ad7 2021-03-12)
% rustc +nightly --version
rustc 1.52.0-nightly (107896c32 2021-03-15)
% cargo +stable build
   Compiling bar v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/bar)
   Compiling foo v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/foo)
   Compiling shared v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/shared)
   Compiling serverctl v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/serverctl)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/home/pnkfelix/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debu\
g/deps/serverctl-642e3662be52c6ef.28rusxpaitpdkpbf.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642e3662be52c6ef.2xx1yk5xpkast9tl.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642\
e3662be52c6ef.2zhtm7t7vp8cvj7s.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642e3662be52c6ef.36qkof5ube8vo2b4.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642e3662be52c6ef.3madvt\
h8zr5q2ma6.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642e3662be52c6ef.8zawlhhi2fxrgml.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642e3662be52c6ef.fexpwlmspuji93d.rcgu.o" "-o\
" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-642e3662be52c6ef" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-L" "/home/pnkfel\
ix/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-lshared" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-lba\
r" "-Wl,--start-group" "-L" "/home/pnkfelix/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-6f77337c1826707d" "-Wl,--end-group" "-Wl,-Bstatic" "/home/pnkfelix/.rustup/toolchains/stable-x86_64-unknown-li\
nux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-d36087076e1dd756.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: /usr/bin/ld: /media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/libshared.so: undefined reference to `bar::bar'
          collect2: error: ld returned 1 exit status


error: aborting due to previous error

error: could not compile `serverctl`

To learn more, run the command again with --verbose.
% cargo +beta build
   Compiling bar v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/bar)
   Compiling foo v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/foo)
   Compiling shared v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/shared)
   Compiling serverctl v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/serverctl)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/home/pnkfelix/.rustup/toolchains/beta-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/\
deps/serverctl-2e2840cf7d082ebe.2tw89ydr2vaopk7g.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e2840cf7d082ebe.343edvd2044ixf22.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e284\
0cf7d082ebe.3nj5822awiovzjf.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e2840cf7d082ebe.3wbp9hhnvmlx8kc8.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e2840cf7d082ebe.4hc0pgqcp\
fq2cts9.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e2840cf7d082ebe.c2mibyzt4rt6ta3.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e2840cf7d082ebe.nj2vcu3x1adrnm9.rcgu.o" "-o" "\
/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-2e2840cf7d082ebe" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-L" "/home/pnkfelix/\
.rustup/toolchains/beta-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-lshared" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-lbar" "-\
Wl,--start-group" "-L" "/home/pnkfelix/.rustup/toolchains/beta-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-6945ccf6d83f593c" "-Wl,--end-group" "-Wl,-Bstatic" "/home/pnkfelix/.rustup/toolchains/beta-x86_64-unknown-linux-gnu/l\
ib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-ab7901d69c9d509b.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: /usr/bin/ld: /media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/libshared.so: undefined reference to `bar::bar'
          collect2: error: ld returned 1 exit status


error: aborting due to previous error

error: could not compile `serverctl`

To learn more, run the command again with --verbose.
% cargo +nightly build
   Compiling bar v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/bar)
   Compiling foo v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/foo)
   Compiling shared v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/shared)
   Compiling serverctl v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/serverctl)
error: cannot satisfy dependencies so `std` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_core` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `libc` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `unwind` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `cfg_if` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `hashbrown` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_alloc` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_demangle` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `addr2line` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `gimli` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `object` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `miniz_oxide` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `adler` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `panic_unwind` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `bar` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: aborting due to 18 previous errors

error: could not compile `serverctl`

To learn more, run the command again with --verbose.
