
+ python2.7 ../x.py dist --host armv7-unknown-linux-gnueabihf,thumbv7neon-unknown-linux-gnueabihf --target armv7-unknown-linux-gnueabihf,thumbv7neon-unknown-linux-gnueabihf
    Finished dev [unoptimized] target(s) in 0.19s
thread 'main' panicked at '

couldn't find required command: "arm-linux-gnueabihf-gcc"

', src/bootstrap/sanity.rs:65:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host armv7-unknown-linux-gnueabihf,thumbv7neon-unknown-linux-gnueabihf --target armv7-unknown-linux-gnueabihf,thumbv7neon-unknown-linux-gnueabihf
Build completed unsuccessfully in 0:00:00
