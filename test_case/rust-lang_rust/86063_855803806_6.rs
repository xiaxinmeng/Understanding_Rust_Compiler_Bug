bash
$ cargo run
    Finished dev [optimized + debuginfo] target(s) in 0.01s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv6m-none-eabi/debug/mcve`
Timer with period zero, disabling
[src/main.rs:9] 1i128 = 1
[src/main.rs:11] x * x = 1

$ cargo run --release
    Finished release [optimized + debuginfo] target(s) in 0.01s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv6m-none-eabi/release/mcve`
Timer with period zero, disabling
[src/main.rs:9] 1i128 = 1
[src/main.rs:11] x * x = 1
