
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: enabling valgrind run-pass tests (CFG_ENABLE_VALGRIND_RPASS)
cfg: no lualatex found, deferring to xelatex
cfg: no xelatex found, deferring to pdflatex
cfg: no pdflatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_borrowck
/home/simon/projects/rust/src/librustc_borrowck/lib.rs:19:40: 19:46 warning: feature has been added to Rust, directive not necessary
/home/simon/projects/rust/src/librustc_borrowck/lib.rs:19 #![feature(default_type_params, globs, if_let, import_shadowing, macro_rules, phase, quote)]
                                                                                                 ^~~~~~
/home/simon/projects/rust/src/librustc_borrowck/lib.rs:20:28: 20:42 warning: feature has been added to Rust, directive not necessary
/home/simon/projects/rust/src/librustc_borrowck/lib.rs:20 #![feature(slicing_syntax, tuple_indexing, unsafe_destructor)]
                                                                                     ^~~~~~~~~~~~~~
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: end <= self.len()', /home/simon/projects/rust/src/libcore/slice.rs:119

stack backtrace:
   1:     0x7f2a3bef9330 - rt::backtrace::imp::write::h966d1e4fbad2eba78zx
   2:     0x7f2a3befb5e0 - failure::on_fail::h84d4e5a9520da465w1x
   3:     0x7f2a3bb4a480 - unwind::begin_unwind_inner::he5c20bf0f48061a4gNc
   4:     0x7f2a3bb4a090 - unwind::begin_unwind_fmt::he573dd74c2408f72DKc
   5:     0x7f2a3bb4a050 - rust_begin_unwind
   6:     0x7f2a3bb9b540 - panicking::panic_fmt::h5504a345757eff91Dul
   7:     0x7f2a3bb99240 - panicking::panic::haa2d14a80e8e27183rl
   8:     0x7f2a3a43c2d0 - metadata::loader::Context<'a>::extract_one::h99c97a961ad0a342xwq
   9:     0x7f2a3a435530 - metadata::loader::Context<'a>::find_library_crate::h0ca00dc18351111bTmq
  10:     0x7f2a3a42dbe0 - metadata::creader::resolve_crate::h83d507b9962a3df2m6o
  11:     0x7f2a3a42a290 - metadata::creader::Env<'a>.visit..Visitor<'v>::visit_view_item::hdcc2fd9ec848ca8ahzo
  12:     0x7f2a3a427ce0 - metadata::creader::read_crates::h0a506fbe34bdbb92byo
  13:     0x7f2a3c320a70 - driver::phase_3_run_analysis_passes::hfe462cb7291c0104Eta
  14:     0x7f2a3c30c850 - driver::compile_input::h5e3b65bed443d1ccrba
  15:     0x7f2a3c49c1b0 - thunk::F.Invoke<A, R>::invoke::h12710441977020552126
  16:     0x7f2a3bed6ea0 - thunk::F.Invoke<A, R>::invoke::h9652845222222678463
  17:     0x7f2a3bb490e0 - task::Task::spawn_thunk::closure.5776
  18:     0x7f2a3bbafab0 - rust_try_inner
  19:     0x7f2a3bbafaa0 - rust_try
  20:     0x7f2a3bb48fe0 - task::Task::run::h106875dadbdd89aeyNb
  21:     0x7f2a3bb489b0 - thunk::F.Invoke<A, R>::invoke::h15203593917914823699
  22:     0x7f2a3bb49d10 - thread::thread_start::hdfa05366eb03ad42S4b
  23:     0x7f2a36ba4250 - start_thread
  24:     0x7f2a3b81f589 - clone
  25:                0x0 - <unknown>

/home/simon/projects/rust/mk/target.mk:165: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_borrowck' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_borrowck] Error 101
