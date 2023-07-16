
pi@raspberrypi:~/chainpon $ cargo build --bin server
   Compiling uinput v0.1.3
error: Could not compile `uinput`.

Caused by:
  process didn't exit successfully: `rustc --crate-name uinput /home/pi/.cargo/registry/src/github.com-1ecc6299db9ec823/uinput-0.1.3/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="libudev"' --cfg 'feature="udev"' -C metadata=de130faeebf090be -C extra-filename=-de130faeebf090be --out-dir /home/pi/chainpon/target/debug/deps -L dependency=/home/pi/chainpon/target/debug/deps --extern custom_derive=/home/pi/chainpon/target/debug/deps/libcustom_derive-3f76e6f2f0571b52.rlib --extern enum_derive=/home/pi/chainpon/target/debug/deps/libenum_derive-04120fea65e83264.rlib --extern libc=/home/pi/chainpon/target/debug/deps/liblibc-af6a89e4ec3b7091.rlib --extern libudev=/home/pi/chainpon/target/debug/deps/liblibudev-79e334ec9ce21a0a.rlib --extern nix=/home/pi/chainpon/target/debug/deps/libnix-96177fe0e83b097c.rlib --extern uinput_sys=/home/pi/chainpon/target/debug/deps/libuinput_sys-71861f0f1d75cf44.rlib --cap-lints allow -L native=/lib/arm-linux-gnueabihf` (signal: 9, SIGKILL: kill)
