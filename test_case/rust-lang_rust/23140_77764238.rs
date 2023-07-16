
$ pwd
/home/japaric/tmp
$ echo 'fn main() {}' > main.rs
$ cd /tmp
$ RUST_BACKTRACE=1 rustc ~/tmp/main.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'can't create relative paths across filesystems', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_back/rpath.rs:115

stack backtrace:
   1:     0x7fc6d98ba2d2 - sys::backtrace::write::h49f84a63c90f3a72QBA
   2:     0x7fc6d98e0322 - panicking::on_panic::hd99ba6230147c149jHJ
   3:     0x7fc6d9826a69 - rt::unwind::begin_unwind_inner::hcba984cddf911653mnJ
   4:     0x7fc6d2daf27c - rt::unwind::begin_unwind::h6703412169040021632
   5:     0x7fc6d2dbf770 - iter::Map<I, F>.Iterator::next::h14795343113335188906
   6:     0x7fc6d2dbd5fa - rpath::get_rpath_flags::h8689aee4bd8a62f8gOa
   7:     0x7fc6d8f51eb6 - back::link::link_args::h0e423504e9a162a3N8a
   8:     0x7fc6d8f4b322 - back::link::link_natively::hecf7c6e78d069d36ZYa
   9:     0x7fc6d8f45d41 - back::link::link_binary::h733fe2e8be711bc3Vua
  10:     0x7fc6d9ee0cbe - driver::phase_6_link_output::hca89f23f478effacwQa
  11:     0x7fc6d9ebaf99 - driver::compile_input::h552c440872c5b657Nba
  12:     0x7fc6d9f7ef1d - run_compiler::h56042a784d7b1fc7G6b
  13:     0x7fc6d9f7cc8c - thunk::F.Invoke<A, R>::invoke::h15113937807686641913
  14:     0x7fc6d9f7b8e0 - rt::unwind::try::try_fn::h14840337430372503284
  15:     0x7fc6d994c0f8 - rust_try_inner
  16:     0x7fc6d994c0e5 - rust_try
  17:     0x7fc6d9f7c06b - thunk::F.Invoke<A, R>::invoke::h6250943894687305386
  18:     0x7fc6d98cdb25 - sys::thread::thread_start::h848d60956d3aea28t8E
  19:     0x7fc6d393a373 - start_thread
  20:     0x7fc6d94af27c - __clone
  21:                0x0 - <unknown>
