
flubba86@localhost:~/rust/src/libstd$ /usr/local/rustjs/bin/cargo build --target=asmjs-unknown-emscripten --release
Adding directories to PATH:
PATH += /mnt/mint/home/flubba86/Downloads/emsdk_portable
PATH += /mnt/mint/home/flubba86/Downloads/emsdk_portable/emscripten/incoming

Setting environment variables:
EM_CONFIG = /home/flubba86/.emscripten
EMSCRIPTEN = /mnt/mint/home/flubba86/Downloads/emsdk_portable/emscripten/incoming

   Compiling gcc v0.3.25
   Compiling libc v0.0.0 (file:///home/flubba86/rust/src/libstd)
   Compiling build_helper v0.1.0 (file:///home/flubba86/rust/src/libstd)
   Compiling core v0.0.0 (file:///home/flubba86/rust/src/libstd)
   Compiling std v0.0.0 (file:///home/flubba86/rust/src/libstd)
Build failed, waiting for other jobs to finish...
failed to run custom build command for `std v0.0.0 (file:///home/flubba86/rust/src/libstd)`
Process didn't exit successfully: `/home/flubba86/rust/src/libstd/target/release/build/std-f43ff7732835f762/build-script-build` (exit code: 1)
--- stdout
cargo:rustc-cfg=cargobuild
cargo:rustc-link-lib=static=backtrace
cargo:rustc-link-search=native=/home/flubba86/rust/src/libstd/target/asmjs-unknown-emscripten/release/build/std-f43ff7732835f762/out/.libs
OPT_LEVEL = Some("3")
PROFILE = Some("release")
TARGET = Some("asmjs-unknown-emscripten")
debug=false opt-level=3
HOST = Some("x86_64-unknown-linux-gnu")
TARGET = Some("asmjs-unknown-emscripten")
TARGET = Some("asmjs-unknown-emscripten")
HOST = Some("x86_64-unknown-linux-gnu")
CC_asmjs-unknown-emscripten = None
CC_asmjs_unknown_emscripten = None
TARGET_CC = None
CC = None
HOST = Some("x86_64-unknown-linux-gnu")
TARGET = Some("asmjs-unknown-emscripten")
HOST = Some("x86_64-unknown-linux-gnu")
CFLAGS_asmjs-unknown-emscripten = None
CFLAGS_asmjs_unknown_emscripten = None
TARGET_CFLAGS = None
CFLAGS = None
running: "sh" "/home/flubba86/rust/src/libstd/../libbacktrace/configure" "--with-pic" "--disable-multilib" "--disable-shared" "--disable-host-shared" "--host=asmjs-unknown-emscripten" "--build=x86_64-unknown-linux-gnu"
checking build system type... x86_64-unknown-linux-gnu
checking host system type... 

command did not execute successfully: "sh" "/home/flubba86/rust/src/libstd/../libbacktrace/configure" "--with-pic" "--disable-multilib" "--disable-shared" "--disable-host-shared" "--host=asmjs-unknown-emscripten" "--build=x86_64-unknown-linux-gnu"
expected success, got: exit code: 1



--- stderr
Invalid configuration `asmjs-unknown-emscripten': system `emscripten' not recognized
configure: error: /bin/sh /home/flubba86/rust/src/libstd/../libbacktrace/config.sub asmjs-unknown-emscripten failed
