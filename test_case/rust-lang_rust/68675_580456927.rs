plain
2020-01-30T20:56:34.5519301Z [RUSTC-TIMING] backtrace test:false 0.286
2020-01-30T20:56:35.1187872Z [RUSTC-TIMING] hashbrown test:false 0.676
2020-01-30T20:56:35.2618371Z warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`
2020-01-30T20:56:35.2618905Z 
2020-01-30T20:56:37.3964798Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2020-01-30T20:56:37.3965900Z   --> src/libstd/sys/sgx/rwlock.rs:16:5
2020-01-30T20:56:37.3966481Z    |
2020-01-30T20:56:37.3967082Z 16 |     mem::transmute::<RWLock, [u8; 128]>(r);
2020-01-30T20:56:37.3968822Z    |
2020-01-30T20:56:37.3968822Z    |
2020-01-30T20:56:37.3969438Z    = note: source type: `sys::sgx::rwlock::RWLock` (1152 bits)
2020-01-30T20:56:37.3970405Z    = note: target type: `[u8; 128]` (1024 bits)
2020-01-30T20:56:38.6432578Z error: aborting due to previous error
2020-01-30T20:56:38.6433415Z 
2020-01-30T20:56:38.6433985Z For more information about this error, try `rustc --explain E0512`.
2020-01-30T20:56:38.6501000Z [RUSTC-TIMING] std test:false 3.528
---
2020-01-30T20:56:38.6676293Z   local time: Thu Jan 30 20:56:38 UTC 2020
2020-01-30T20:56:38.9327518Z   network time: Thu, 30 Jan 2020 20:56:38 GMT
2020-01-30T20:56:38.9328065Z == end clock drift check ==
2020-01-30T20:56:40.3147051Z 
2020-01-30T20:56:40.3228344Z ##[error]Bash exited with code '1'.
2020-01-30T20:56:40.3281434Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-30T20:56:40.3284081Z ==============================================================================
2020-01-30T20:56:40.3284157Z Task         : Get sources
2020-01-30T20:56:40.3284244Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
