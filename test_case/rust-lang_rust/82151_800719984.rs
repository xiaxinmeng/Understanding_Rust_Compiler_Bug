
21-13-22 issue_82151/dylib-errors (git:error/nested) [ERROR#1] % ( RUSTC=/home/pnkfelix/.rustup/toolchains/nightly-2021-03-15-x86_64-unknown-linux-gnu/bin/rustc ; V=nightly-2021-03-14 ; cargo +$V clean ; cargo +$V -\
-version ; cargo +$V build  )
cargo 1.52.0-nightly (970bc67c3 2021-03-07)
   Compiling bar v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/bar)
   Compiling foo v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/foo)
   Compiling shared v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/shared)
   Compiling serverctl v0.1.0 (/media/pnkfelix/Rust/issue_82151/dylib-errors/serverctl)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/home/pnkfelix/.rustup/toolchains/nightly-2021-03-14-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/media\
/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-6c04b63e1d5ee98d.266n9v3ct69lmd5x.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-6c04b63e1d5ee98d.36w4j5atbcbca\
9ft.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-6c04b63e1d5ee98d.4aoqsl8y7dke2vsk.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-6c04b63e1d5\
ee98d.4kwx94anssc9hetr.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-6c04b63e1d5ee98d.54ihl5vihuy8m9gk.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/se\
rverctl-6c04b63e1d5ee98d.gssdicypgme7176.rcgu.o" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/serverctl-6c04b63e1d5ee98d.r966a0ktyvsy5up.rcgu.o" "-o" "/media/pnkfelix/Rust/issue_82151/dylib-error\
s/target/debug/deps/serverctl-6c04b63e1d5ee98d" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-L" "/home/pnkfelix/.rust\
up/toolchains/nightly-2021-03-14-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps" "-lshared" "-L" "/media/pnkfelix/Rust/issue_\
82151/dylib-errors/target/debug/deps" "-lbar" "-Wl,--start-group" "-L" "/home/pnkfelix/.rustup/toolchains/nightly-2021-03-14-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-3d933644112ba794\
" "-Wl,--end-group" "-Wl,-Bstatic" "/home/pnkfelix/.rustup/toolchains/nightly-2021-03-14-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-761b290f47712921.rlib" "-Wl,-Bdynamic" \
"-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: /usr/bin/ld: /media/pnkfelix/Rust/issue_82151/dylib-errors/target/debug/deps/libshared.so: undefined reference to `bar::bar'
          collect2: error: ld returned 1 exit status


error: aborting due to previous error

error: could not compile `serverctl`

To learn more, run the command again with --verbose.
21-13-44 issue_82151/dylib-errors (git:error/nested) [ERROR#1] % ( RUSTC=/home/pnkfelix/.rustup/toolchains/nightly-2021-03-15-x86_64-unknown-linux-gnu/bin/rustc ; V=nightly-2021-03-15 ; cargo +$V clean ; cargo +$V -\
-version ; cargo +$V build  )
cargo 1.52.0-nightly (32da9eaa5 2021-03-13)
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
