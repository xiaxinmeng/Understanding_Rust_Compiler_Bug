
% RUSTFLAGS="-C target-cpu=native" cargo build --verbose --release
   Compiling winapi v0.2.8 (file:///tmp/winapi-rs)
   Compiling winapi-build v0.1.1 (file:///tmp/winapi-rs/build)
     Running `rustc --crate-name winapi /tmp/winapi-rs/src/lib.rs --crate-type lib -C opt-level=3 -C metadata=ab96afe94df678e7 -C extra-filename=-ab96afe94df678e7 --out-dir /tmp/winapi-rs/lib/kernel32/target/release/deps --emit=dep-info,link -L dependency=/tmp/winapi-rs/lib/kernel32/target/release/deps -C target-cpu=native`
     Running `rustc --crate-name build /tmp/winapi-rs/build/src/lib.rs --crate-type lib -C opt-level=3 -C metadata=0ccc9c9bd237e078 -C extra-filename=-0ccc9c9bd237e078 --out-dir /tmp/winapi-rs/lib/kernel32/target/release/deps --emit=dep-info,link -L dependency=/tmp/winapi-rs/lib/kernel32/target/release/deps -C target-cpu=native`
   Compiling kernel32-sys v0.2.2 (file:///tmp/winapi-rs/lib/kernel32)
     Running `rustc --crate-name build_script_build build.rs --crate-type bin -C opt-level=3 -C metadata=70200ca6b7279963 -C extra-filename=-70200ca6b7279963 --out-dir /tmp/winapi-rs/lib/kernel32/target/release/build/kernel32-sys-70200ca6b7279963 --emit=dep-info,link -L dependency=/tmp/winapi-rs/lib/kernel32/target/release/deps --extern build=/tmp/winapi-rs/lib/kernel32/target/release/deps/libbuild-0ccc9c9bd237e078.rlib -C target-cpu=native`
     Running `/tmp/winapi-rs/lib/kernel32/target/release/build/kernel32-sys-70200ca6b7279963/build-script-build`
error: failed to run custom build command for `kernel32-sys v0.2.2 (file:///tmp/winapi-rs/lib/kernel32)`
process didn't exit successfully: `/tmp/winapi-rs/lib/kernel32/target/release/build/kernel32-sys-70200ca6b7279963/build-script-build` (signal: 4, SIGILL: illegal instruction)
