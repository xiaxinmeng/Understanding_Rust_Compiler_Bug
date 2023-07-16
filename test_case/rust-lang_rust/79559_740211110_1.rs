
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/home/jonathan/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/jonathan/src/langstartbug/target/debug/examples/teststart-c94a695dacb81923.150bybnj090jvy04.rcgu.o" "/home/jonathan/src/langstartbug/target/debug/examples/teststart-c94a695dacb81923.mnpwjudxw992adt.rcgu.o" "-o" "/home/jonathan/src/langstartbug/target/debug/examples/teststart-c94a695dacb81923" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/home/jonathan/src/langstartbug/target/debug/deps" "-L" "/home/jonathan/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/home/jonathan/src/langstartbug/target/debug/deps/liblangstart-57411b13d382a48d.rlib" "/home/jonathan/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-90996f4879673567.rlib" "/home/jonathan/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9ea09a899c3eda46.rlib" "-Wl,--end-group" "/home/jonathan/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-ef2408da76957905.rlib" "-Wl,-Bdynamic"
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/10.2.0/../../../../lib/Scrt1.o: in function `_start':
          (.text+0x16): undefined reference to `__libc_csu_fini'
          /usr/bin/ld: (.text+0x1d): undefined reference to `__libc_csu_init'
          /usr/bin/ld: (.text+0x2a): undefined reference to `__libc_start_main'
          collect2: error: ld returned 1 exit status


