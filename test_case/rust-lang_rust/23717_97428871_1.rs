
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g --crate-type lib -L libcore-thumbv7m src/lib.rs
