andrei@raspberrypi:~/repos/testy $ export RUSTFLAGS=""
andrei@raspberrypi:~/repos/testy $ echo ${RUSTFLAGS}

andrei@raspberrypi:~/repos/testy $ cargo -V
cargo 1.37.0 (9edd08916 2019-08-02)
andrei@raspberrypi:~/repos/testy $ rm -r Cargo.lock target
andrei@raspberrypi:~/repos/testy $ cargo build --release
    Updating crates.io index
   Compiling libc v0.2.62
   Compiling testy v0.1.0 (/home/andrei/repos/testy)
    Finished release [optimized] target(s) in 19.87s
andrei@raspberrypi:~/repos/testy $ rustup default beta
info: using existing install for 'beta-armv7-unknown-linux-gnueabihf'
info: default toolchain set to 'beta-armv7-unknown-linux-gnueabihf'

  beta-armv7-unknown-linux-gnueabihf unchanged - rustc 1.38.0-beta.1 (e450539c2 2019-08-13)

andrei@raspberrypi:~/repos/testy $ cargo -V
cargo 1.38.0-beta (e853aa976 2019-08-09)
andrei@raspberrypi:~/repos/testy $ rm -r Cargo.lock target
andrei@raspberrypi:~/repos/testy $ cargo build --release
    Updating crates.io index
   Compiling libc v0.2.62
error: Could not compile `libc`.

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build /home/andrei/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.62/build.rs --color always --crate-type bin --emit=dep-info,link -C opt-level=3 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=2098fa4b2feb2f48 -C extra-filename=-2098fa4b2feb2f48 --out-dir /home/andrei/repos/testy/target/release/build/libc-2098fa4b2feb2f48 -L dependency=/home/andrei/repos/testy/target/release/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
andrei@raspberrypi:~/repos/testy $ rustup default nightly
info: using existing install for 'nightly-armv7-unknown-linux-gnueabihf'
info: default toolchain set to 'nightly-armv7-unknown-linux-gnueabihf'

  nightly-armv7-unknown-linux-gnueabihf unchanged - rustc 1.38.0-nightly (c43d03a19 2019-08-14)

andrei@raspberrypi:~/repos/testy $ cargo -V
cargo 1.38.0-nightly (e853aa976 2019-08-09)
andrei@raspberrypi:~/repos/testy $ rm -r Cargo.lock target/
andrei@raspberrypi:~/repos/testy $ cargo build --release
    Updating crates.io index
   Compiling libc v0.2.62
error: Could not compile `libc`.

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build /home/andrei/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.62/build.rs --color always --crate-type bin --emit=dep-info,link -C opt-level=3 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=a233ac8b73169014 -C extra-filename=-a233ac8b73169014 --out-dir /home/andrei/repos/testy/target/release/build/libc-a233ac8b73169014 -L dependency=/home/andrei/repos/testy/target/release/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)





andrei@raspberrypi:~/repos/testy $ export RUSTFLAGS="-Ccodegen-units=1"
andrei@raspberrypi:~/repos/testy $ echo ${RUSTFLAGS}
-Ccodegen-units=1
andrei@raspberrypi:~/repos/testy $ rustup default stable
info: using existing install for 'stable-armv7-unknown-linux-gnueabihf'
info: default toolchain set to 'stable-armv7-unknown-linux-gnueabihf'

  stable-armv7-unknown-linux-gnueabihf unchanged - rustc 1.37.0 (eae3437df 2019-08-13)

andrei@raspberrypi:~/repos/testy $ cargo -V
cargo 1.37.0 (9edd08916 2019-08-02)
andrei@raspberrypi:~/repos/testy $ rm -r Cargo.lock target/
andrei@raspberrypi:~/repos/testy $ cargo build --release
    Updating crates.io index
   Compiling libc v0.2.62
   Compiling testy v0.1.0 (/home/andrei/repos/testy)
    Finished release [optimized] target(s) in 12.95s
andrei@raspberrypi:~/repos/testy $ rustup default beta
info: using existing install for 'beta-armv7-unknown-linux-gnueabihf'
info: default toolchain set to 'beta-armv7-unknown-linux-gnueabihf'

  beta-armv7-unknown-linux-gnueabihf unchanged - rustc 1.38.0-beta.1 (e450539c2 2019-08-13)

andrei@raspberrypi:~/repos/testy $ cargo -V
cargo 1.38.0-beta (e853aa976 2019-08-09)
andrei@raspberrypi:~/repos/testy $ rm -r Cargo.lock target/
andrei@raspberrypi:~/repos/testy $ cargo build --release
    Updating crates.io index
   Compiling libc v0.2.62
   Compiling testy v0.1.0 (/home/andrei/repos/testy)
    Finished release [optimized] target(s) in 12.94s
andrei@raspberrypi:~/repos/testy $ rustup default nightly
info: using existing install for 'nightly-armv7-unknown-linux-gnueabihf'
info: default toolchain set to 'nightly-armv7-unknown-linux-gnueabihf'

  nightly-armv7-unknown-linux-gnueabihf unchanged - rustc 1.38.0-nightly (c43d03a19 2019-08-14)

andrei@raspberrypi:~/repos/testy $ cargo -V
cargo 1.38.0-nightly (e853aa976 2019-08-09)
andrei@raspberrypi:~/repos/testy $ rm -r Cargo.lock target/
andrei@raspberrypi:~/repos/testy $ cargo build --release
    Updating crates.io index
   Compiling libc v0.2.62
   Compiling testy v0.1.0 (/home/andrei/repos/testy)
    Finished release [optimized] target(s) in 13.02s