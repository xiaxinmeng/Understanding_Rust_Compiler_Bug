
cargo rustc -Z build-std=core --target=thumbv7m-none-eabi -- -Clinker=arm-none-eabi-gcc -C link-arg=-nostartfiles -C link-arg=-Tlink.x
