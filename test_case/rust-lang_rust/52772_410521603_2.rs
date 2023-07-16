
[root@s7netserver rustc-1.27.2-src]# ./x.py build && sudo ./x.py install
      <output deleted>
failed to run: /root/dev/rust/rustc-1.27.2-src/build/i686-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /root/dev/rust/rustc-1.27.2-src/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:01:06
[root@s7netserver rustc-1.27.2-src]# /root/dev/rust/rustc-1.27.2-src/build/i686-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /root/dev/rust/rustc-1.27.2-src/src/bootstrap/Cargo.toml
Segmentation fault
