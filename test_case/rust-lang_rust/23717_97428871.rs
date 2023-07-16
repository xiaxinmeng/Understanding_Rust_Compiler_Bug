
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/libcore/lib.rs --out-dir libs-arm
rustc --cfg feature=\"external_funcs\" -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/liballoc/lib.rs --out-dir libs-arm -L libs-arm
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/libunicode/lib.rs --out-dir libs-arm -L libs-arm
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/libcollections/lib.rs --out-dir libs-arm -L libs-arm
