
mkdir -p /home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
cd /home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu && rm -rf cortex-m-rt
cd /home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu && bash -x /home/sekineh/rustme12/src/test/run-make/thumb-none-qemu/../git_clone_sha1.sh cortex-m-rt https://github.com/rust-embedded/cortex-m-rt 62972c8a89ff54b76f9ef0d600c1fcf7a233aabd
HEAD is now at 62972c8 Merge pull request #129 from rust-embedded/release
cd /home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/cortex-m-rt && /home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv7m-none-eabi --example qemu           | grep "x = 42"
Makefile:25: recipe for target 'all' failed
