
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(+0x73cac3)[0x7fd19f513ac3]
/lib/x86_64-linux-gnu/libc.so.6(+0x3cef0)[0x7fd19eaa0ef0]
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(_RNvMs1Q_NtNtCseIS259k75xz_12rustc_middle2ty7contextNtB6_6TyCtxt13intern_layout+0x24)[0x7fd1a1501b84]
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(+0xade075)[0x7fd19f8b5075]
[a lot of errors like this]
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(+0xb851a4)[0x7fd19f95c1a4]
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(+0xb9202e)[0x7fd19f96902e]
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(+0xbd7c45)[0x7fd19f9aec45]
/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-47d0b6818de97398.so(+0xb851a4)[0x7fd19f95c1a4]
rustc exited with signal: 11 (core dumped)
error: could not compile `core`

Caused by:
  process didn't exit successfully: `/home/r/src/rust/rustc/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=8 -C debuginfo=1 -C metadata=d2ad3ad0cb571346 -C extra-filename=-d2ad3ad0cb571346 --out-dir /home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zsave-analysis -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:00:32
