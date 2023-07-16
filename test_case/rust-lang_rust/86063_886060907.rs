
qemu-system-arm -cpu cortex-m3 -M lm3s6965evb -kernel bug-example/target/thumbv6m-none-eabi/debug/bug-example -gdb tcp::3333 -S --semihosting-config enable=on,target=native -nographic
