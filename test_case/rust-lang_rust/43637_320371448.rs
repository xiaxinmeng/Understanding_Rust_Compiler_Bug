
   Compiling rustc_errors v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_errors)
error: cannot satisfy dependencies so `std` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rand` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `std_unicode` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc_system` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `libc` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `unwind` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc_jemalloc` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `panic_unwind` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: aborting due to 11 previous errors

error: Could not compile `rustc_errors`.

Caused by:
  process didn't exit successfully: `/home/r/src/rust/rustc.2/build/bootstrap/debug/rustc --crate-name rustc_errors src/librustc_errors/lib.rs --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8861e0bae2d701bf -C extra-filename=-8861e0bae2d701bf --out-dir /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern serialize=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-7949b697b8b9ae91.so --extern serialize=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-7949b697b8b9ae91.rlib --extern syntax_pos=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-60df18e62faffff4.so` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
error: build failed
