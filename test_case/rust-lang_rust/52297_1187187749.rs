
> CC=powerpc-linux-gnu-gcc CARGO_TARGET_POWERPC_UNKNOWN_LINUX_GNU_LINKER=powerpc-linux-gnu-gcc cargo build --target=powerpc-unknown-linux-gnu
[...]
> qemu-ppc -L/usr/powerpc-linux-gnu/ ./target/powerpc-unknown-linux-gnu/debug/reproduce 
x=7fffffff y=ffffffff
