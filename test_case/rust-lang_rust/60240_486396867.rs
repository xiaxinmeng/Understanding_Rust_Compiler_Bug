plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:027cce68
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:13:49] Step 8/11 : ENV HOSTS=x86_64-unknown-linux-musl
[00:13:49]  ---> Running in a26d952b2f61
[00:13:50] Removing intermediate container a26d952b2f61
[00:13:50]  ---> d87b354f036f
[00:13:50] Step 9/11 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs       --set target.x86_64-unknown-linux-musl.crt-static=false       --build $HOSTS       --set target.x86_64-unknown-linux-musl.cc=x86_64-linux-musl-gcc       --set target.x86_64-unknown-linux-musl.cxx=x86_64-linux-musl-g++       --set target.x86_64-unknown-linux-musl.linker=x86_64-linux-musl-gcc
[00:13:50] Removing intermediate container 363d3d999733
[00:13:50]  ---> 608a6747e2a6
[00:13:50] Step 10/11 : ENV CFLAGS_x86_64_unknown_linux_musl="-Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none     -Wl,--compress-debug-sections=none"
[00:13:50]  ---> Running in 276c37783adf
