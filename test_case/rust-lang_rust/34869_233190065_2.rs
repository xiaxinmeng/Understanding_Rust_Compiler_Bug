
~/D/pq-sys git:master ❯❯❯ rustup override set beta                                                                                                          ✱
info: using existing install for 'beta-x86_64-apple-darwin'
info: override toolchain for '/Users/chris/Documents/pq-sys' set to 'beta-x86_64-apple-darwin'

  beta-x86_64-apple-darwin unchanged - rustc 1.11.0-beta.1 (8dc253bcf 2016-07-05)

~/D/pq-sys git:master ❯❯❯ cargo test --verbose                                                                                                              ✱
   Compiling pq-sys v0.2.1 (file:///Users/chris/Documents/pq-sys)
   Compiling libc v0.2.14
     Running `rustc build.rs --crate-name build_script_build --crate-type bin -g --out-dir /Users/chris/Documents/pq-sys/target/debug/build/pq-sys-683de9b8454d5f4b --emit=dep-info,link -L dependency=/Users/chris/Documents/pq-sys/target/debug -L dependency=/Users/chris/Documents/pq-sys/target/debug/deps`
     Running `rustc /Users/chris/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.14/src/lib.rs --crate-name libc --crate-type lib -g --cfg feature=\"default\" --cfg feature=\"use_std\" -C metadata=1f3392fe1afd1313 -C extra-filename=-1f3392fe1afd1313 --out-dir /Users/chris/Documents/pq-sys/target/debug/deps --emit=dep-info,link -L dependency=/Users/chris/Documents/pq-sys/target/debug/deps -L dependency=/Users/chris/Documents/pq-sys/target/debug/deps --cap-lints allow`
     Running `/Users/chris/Documents/pq-sys/target/debug/build/pq-sys-683de9b8454d5f4b/build-script-build`
     Running `rustc src/lib.rs --crate-name pq_sys -C codegen-units=1 -g --test -C metadata=5b5cdff68186daa5 -C extra-filename=-5b5cdff68186daa5 --out-dir /Users/chris/Documents/pq-sys/target/debug --emit=dep-info,link -L dependency=/Users/chris/Documents/pq-sys/target/debug -L dependency=/Users/chris/Documents/pq-sys/target/debug/deps --extern libc=/Users/chris/Documents/pq-sys/target/debug/deps/liblibc-1f3392fe1afd1313.rlib -L native=/usr/local/lib`
     Running `rustc src/lib.rs --crate-name pq_sys --crate-type lib -g --out-dir /Users/chris/Documents/pq-sys/target/debug --emit=dep-info,link -L dependency=/Users/chris/Documents/pq-sys/target/debug -L dependency=/Users/chris/Documents/pq-sys/target/debug/deps --extern libc=/Users/chris/Documents/pq-sys/target/debug/deps/liblibc-1f3392fe1afd1313.rlib -L native=/usr/local/lib`
     Running `/Users/chris/Documents/pq-sys/target/debug/pq_sys-5b5cdff68186daa5`
dyld: Symbol not found: __cg_jpeg_resync_to_restart
  Referenced from: /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
  Expected in: /usr/local/lib/libJPEG.dylib
 in /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
error: Process didn't exit successfully: `/Users/chris/Documents/pq-sys/target/debug/pq_sys-5b5cdff68186daa5` (signal: 5, SIGTRAP: trace/breakpoint trap)

Caused by:
  Process didn't exit successfully: `/Users/chris/Documents/pq-sys/target/debug/pq_sys-5b5cdff68186daa5` (signal: 5, SIGTRAP: trace/breakpoint trap)
