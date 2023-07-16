
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "--target=x86_64-fuchsia" "--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot" "-I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-ebbca5a578951bf7/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
  cargo:warning=[2022-08-06T23:07:37Z TRACE sccache::cmdline] parse
  cargo:warning=[2022-08-06T23:07:37Z DEBUG sccache::config] Attempting to read config file at "/home/user/.config/sccache/config"
  cargo:warning=[2022-08-06T23:07:37Z DEBUG sccache::config] Couldn't open config file: No such file or directory (os error 2)
  cargo:warning=[2022-08-06T23:07:37Z TRACE sccache::commands] Command::Compile { "x86_64-fuchsia-clang", ["-O3", "-ffunction-sections", "-fdata-sections", "-fPIC", "-g", "-fno-omit-frame-pointer", "--target=x86_64-fuchsia", "-ffunction-sections", "-fdata-sections", "-fPIC", "--target=x86_64-fuchsia", "--target=x86_64-fuchsia", "--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot", "-I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include", "-fno-builtin", "-fvisibility=hidden", "-ffreestanding", "-DVISIBILITY_HIDDEN", "-o", "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-ebbca5a578951bf7/out/absvdi2.o", "-c", "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"], "/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.73" }
  cargo:warning=[2022-08-06T23:07:37Z TRACE sccache::commands] connect_or_start_server(4226)
  cargo:warning=[2022-08-06T23:07:37Z TRACE sccache::client] connect_to_server(4226)
  cargo:warning=[2022-08-06T23:07:37Z TRACE mio::poll] registering with poller
  cargo:warning=[2022-08-06T23:07:37Z TRACE mio::poll] registering with poller
  cargo:warning=[2022-08-06T23:07:37Z TRACE sccache::commands] do_compile
  cargo:warning=[2022-08-06T23:07:37Z TRACE tokio_reactor] event Readable Token(4194303)
  cargo:warning=[2022-08-06T23:07:37Z TRACE tokio_reactor] loop process - 1 events, 0.000s
  cargo:warning=sccache: error: failed to execute compile
  cargo:warning=sccache: caused by: cannot find binary path
  exit status: 2
