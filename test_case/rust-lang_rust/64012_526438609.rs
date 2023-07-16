plain
2019-08-30T03:06:20.9956390Z [RUSTC-TIMING] rand_pcg test:false 0.373
2019-08-30T03:06:21.5581520Z [RUSTC-TIMING] rand_chacha test:false 0.678
2019-08-30T03:06:22.4342180Z [RUSTC-TIMING] crates_io test:false 10.259
2019-08-30T03:06:22.6068400Z [RUSTC-TIMING] build_script_main test:false 4.424
2019-08-30T03:06:27.4475160Z error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:42: could not fully normalize `<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0> as sized_chunks::types::ChunkLength<A>>::SizedType`
2019-08-30T03:06:27.4575360Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-30T03:06:27.4675560Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T03:06:27.5134610Z [RUSTC-TIMING] rand test:false 5.949
2019-08-30T03:06:27.5237540Z    Compiling tempfile v3.0.5
2019-08-30T03:06:27.5237540Z    Compiling tempfile v3.0.5
2019-08-30T03:06:27.5243760Z error: aborting due to previous error
2019-08-30T03:06:27.5244460Z 
2019-08-30T03:06:27.5349890Z 
2019-08-30T03:06:27.5451030Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T03:06:27.5551320Z 
2019-08-30T03:06:27.5653280Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T03:06:27.5752620Z 
2019-08-30T03:06:27.5854480Z note: rustc 1.39.0-nightly (5aeb8941f 2019-08-30) running on x86_64-apple-darwin
2019-08-30T03:06:27.5954580Z 
2019-08-30T03:06:27.6056670Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z osx-rpath-install-name -C opt-level=2 -C debuginfo=0 -C debug-assertions=n -C link-args=-Wl,-rpath,@loader_path/../lib --crate-type lib
2019-08-30T03:06:27.6157690Z 
2019-08-30T03:06:27.6204160Z note: some of the compiler flags provided by cargo are hidden
2019-08-30T03:06:27.6311540Z [RUSTC-TIMING] im_rc test:false 6.456
2019-08-30T03:06:27.6313600Z error: Could not compile `im-rc`.
2019-08-30T03:06:27.6315050Z warning: build failed, waiting for other jobs to finish...
2019-08-30T03:06:27.6316690Z [RUSTC-TIMING] parking_lot_core test:false 2.576
---
2019-08-30T03:07:35.1063790Z == clock drift check ==
2019-08-30T03:07:35.1131940Z   local time: Fri Aug 30 03:07:35 UTC 2019
2019-08-30T03:07:35.1850640Z   network time: Fri, 30 Aug 2019 03:07:35 GMT
2019-08-30T03:07:35.1853350Z == end clock drift check ==
2019-08-30T03:07:35.2038590Z ##[error]Bash exited with code '1'.
2019-08-30T03:07:35.2095690Z ##[section]Starting: Upload CPU usage statistics
2019-08-30T03:07:35.2100420Z ==============================================================================
2019-08-30T03:07:35.2100540Z Task         : Bash
2019-08-30T03:07:35.2100630Z Description  : Run a Bash script on macOS, Linux, or Windows
