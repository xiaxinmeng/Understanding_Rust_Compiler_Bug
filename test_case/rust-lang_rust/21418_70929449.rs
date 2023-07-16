
# stage0 liballoc with assumptions compiled in
$ time ./x86_64-unknown-linux-gnu/stage0/bin/rustc -O ./src/librustc/lib.rs --out-dir foo
./x86_64-unknown-linux-gnu/stage0/bin/rustc -O ./src/librustc/lib.rs --out-di  610.62s user 2.01s system 99% cpu 10:12.68 total

# stage0 liballoc without assumptions compiled in
$ time ./x86_64-unknown-linux-gnu/stage0/bin/rustc -O ./src/librustc/lib.rs --out-dir foo
./x86_64-unknown-linux-gnu/stage0/bin/rustc -O ./src/librustc/lib.rs --out-di  202.90s user 1.70s system 99% cpu 3:24.62 total

