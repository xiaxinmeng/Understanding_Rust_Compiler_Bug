
╭ ➜ ben@archlinux:/tmp/scratch
╰ ➤ cross run --target mips-unknown-linux-gnu --verbose
+ cargo metadata --format-version 1 --filter-platform mips-unknown-linux-gnu
+ rustc --print sysroot
+ rustup toolchain list
+ rustup target list --toolchain stable-x86_64-unknown-linux-gnu
+ rustup component list --toolchain stable-x86_64-unknown-linux-gnu
+ /usr/bin/docker
+ /usr/bin/docker run --userns host -e 'PKG_CONFIG_ALLOW_CROSS=1' -e 'XARGO_HOME=/xargo' -e 'CARGO_HOME=/cargo' -e 'CARGO_TARGET_DIR=/target' -e 'CROSS_RUNNER=' -e TERM -e 'USER=ben' --rm --user 1000:1000 -v /home/ben/.xargo:/xargo:Z -v /home/ben/.cargo:/cargo:Z -v /cargo/bin -v /tmp/scratch:/project:Z -v /home/ben/.rustup/toolchains/stable-x86_64-unknown-linux-gnu:/rust:Z,ro -v /tmp/scratch/target:/target:Z -w /project -i -t ghcr.io/cross-rs/mips-unknown-linux-gnu:0.2.4 sh -c 'PATH=$PATH:/rust/bin cargo run --target mips-unknown-linux-gnu --verbose'
   Compiling scratch v0.1.0 (/project)
     Running `rustc --crate-name scratch --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=415e3bf8bc61106d -C extra-filename=-415e3bf8bc61106d --out-dir /target/mips-unknown-linux-gnu/debug/deps --target mips-unknown-linux-gnu -C linker=mips-linux-gnu-gcc -C incremental=/target/mips-unknown-linux-gnu/debug/incremental -L dependency=/target/mips-unknown-linux-gnu/debug/deps -L dependency=/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `/linux-runner mips /target/mips-unknown-linux-gnu/debug/scratch`
╭ ➜ ben@archlinux:/tmp/scratch
╰ ➤ echo $?
0
