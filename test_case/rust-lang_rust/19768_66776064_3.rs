
$ env RUST_BACKTRACE=1 cargo build --verbose                                                                                                                     ~/d/t/rust-ice-19768 130
   Compiling nalgebra v0.1.0 (https://github.com/sebcrozet/nalgebra#6c431ff6)
     Running `rustc src/lib.rs --crate-name nalgebra --crate-type lib -g -C metadata=d0bd0af049b73250 -C extra-filename=-d0bd0af049b73250 --out-dir /home/eduard/dev/toy/rust-ice-19768/target/deps --dep-info /home/eduard/dev/toy/rust-ice-19768/target/.fingerprint/nalgebra-d0bd0af049b73250/dep-lib-nalgebra -L /home/eduard/dev/toy/rust-ice-19768/target/deps -L /home/eduard/dev/toy/rust-ice-19768/target/deps -Awarnings`
       Fresh quickcheck v0.1.5 (https://github.com/BurntSushi/quickcheck#d7daf088)
       Fresh quickcheck_macros v0.1.5 (https://github.com/BurntSushi/quickcheck#d7daf088)
   Compiling acacia v0.0.1 (https://github.com/aepsil0n/acacia#bd2468e5)
     Running `rustc /home/eduard/.cargo/git/checkouts/acacia-c7b632136de50e89/master/src/lib.rs --crate-name acacia --crate-type lib -g -C metadata=44130cf3227ab7ae -C extra-filename=-44130cf3227ab7ae --out-dir /home/eduard/dev/toy/rust-ice-19768/target/deps --dep-info /home/eduard/dev/toy/rust-ice-19768/target/.fingerprint/acacia-44130cf3227ab7ae/dep-lib-acacia -L /home/eduard/dev/toy/rust-ice-19768/target/deps -L /home/eduard/dev/toy/rust-ice-19768/target/deps --extern quickcheck_macros=/home/eduard/dev/toy/rust-ice-19768/target/deps/libquickcheck_macros-45b5ea34e15bd1a0.so --extern nalgebra=/home/eduard/dev/toy/rust-ice-19768/target/deps/libnalgebra-d0bd0af049b73250.rlib --extern quickcheck=/home/eduard/dev/toy/rust-ice-19768/target/deps/libquickcheck-06670abe8b1a8157.rlib -Awarnings`
   Compiling rust-ice-19768 v0.0.1 (file:///home/eduard/dev/toy/rust-ice-19768)
     Running `rustc /home/eduard/dev/toy/rust-ice-19768/src/lib.rs --crate-name rust-ice-19768 --crate-type lib -g -C metadata=48de707623d5662d -C extra-filename=-48de707623d5662d --out-dir /home/eduard/dev/toy/rust-ice-19768/target --dep-info /home/eduard/dev/toy/rust-ice-19768/target/.fingerprint/rust-ice-19768-48de707623d5662d/dep-lib-rust-ice-19768 -L /home/eduard/dev/toy/rust-ice-19768/target -L /home/eduard/dev/toy/rust-ice-19768/target/deps --extern nalgebra=/home/eduard/dev/toy/rust-ice-19768/target/deps/libnalgebra-d0bd0af049b73250.rlib --extern acacia=/home/eduard/dev/toy/rust-ice-19768/target/deps/libacacia-44130cf3227ab7ae.rlib`
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: end <= self.len()', /build/rust-git/src/rust/src/libcore/slice.rs:432

stack backtrace:
   1:     0x7f380a62d210 - rt::backtrace::imp::write::h49d8ac939d8781e7GOx
   2:     0x7f380a630310 - <unknown>
   3:     0x7f380a28c740 - unwind::begin_unwind_inner::h0288b255d3ea1869CJc
   4:     0x7f380a28c240 - unwind::begin_unwind_fmt::hf57382361e769392ZGc
   5:     0x7f380a28c200 - rust_begin_unwind
   6:     0x7f380a2cce20 - panicking::panic_fmt::hbf754a34cfe87a0eRtl
   7:     0x7f380a2cab20 - panicking::panic::hb1246f74b69e7954hrl
   8:     0x7f3808c56a70 - <unknown>
   9:     0x7f3808c4f6d0 - <unknown>
  10:     0x7f3808c4ad70 - metadata::loader::Context<'a>::load_library_crate::hddfb7c19c92a7307Rew
  11:     0x7f3808c45da0 - <unknown>
  12:     0x7f3808c48170 - <unknown>
  13:     0x7f3808c45da0 - <unknown>
  14:     0x7f3808c40ef0 - metadata::creader::Env<'a>.visit..Visitor<'v>::visit_view_item::h070c7cbec1c3f414pzu
  15:     0x7f3808c40380 - metadata::creader::read_crates::h20d08d8c8148ca69jyu
  16:     0x7f380aa9b2f0 - <unknown>
  17:     0x7f380aa5c250 - driver::phase_3_run_analysis_passes::h04cc27406c5d68a4Cta
  18:     0x7f380aa4b5e0 - driver::compile_input::hb6e5a7fd0e2cfd7cpba
  19:     0x7f380aae6970 - <unknown>
  20:     0x7f380aae6860 - <unknown>
  21:     0x7f380aaf80b0 - <unknown>
  22:     0x7f380a607160 - <unknown>
  23:     0x7f380a28a3e0 - <unknown>
  24:     0x7f380a2dfb90 - <unknown>
  25:     0x7f380a2dfb80 - rust_try
  26:     0x7f380a28a4c0 - unwind::try::h77d5b9b394dc26e4Tyc
  27:     0x7f380a28a280 - task::Task::run::hc596a395d876c158fKb
  28:     0x7f380a289e70 - <unknown>
  29:     0x7f380a28b8c0 - <unknown>
  30:     0x7f38056b2250 - start_thread
  31:     0x7f3809f61589 - clone
  32:                0x0 - <unknown>

Could not compile `rust-ice-19768`.

Caused by:
  Process didn't exit successfully: `rustc /home/eduard/dev/toy/rust-ice-19768/src/lib.rs --crate-name rust-ice-19768 --crate-type lib -g -C metadata=48de707623d5662d -C extra-filename=-48de707623d5662d --out-dir /home/eduard/dev/toy/rust-ice-19768/target --dep-info /home/eduard/dev/toy/rust-ice-19768/target/.fingerprint/rust-ice-19768-48de707623d5662d/dep-lib-rust-ice-19768 -L /home/eduard/dev/toy/rust-ice-19768/target -L /home/eduard/dev/toy/rust-ice-19768/target/deps --extern nalgebra=/home/eduard/dev/toy/rust-ice-19768/target/deps/libnalgebra-d0bd0af049b73250.rlib --extern acacia=/home/eduard/dev/toy/rust-ice-19768/target/deps/libacacia-44130cf3227ab7ae.rlib` (status=101)
