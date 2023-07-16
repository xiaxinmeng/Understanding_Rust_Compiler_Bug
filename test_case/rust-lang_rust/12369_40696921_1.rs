
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'called `Option::unwrap()` on a `None` value', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libstd/option.rs:264
stack backtrace:
   1:     0x7fde205896a0 - rt::backtrace::imp::write::h167f38c59baeff8aqRa::v0.11.pre
   2:     0x7fde204eafa0 - rt::unwind::begin_unwind_inner::hc4665026b45f6a5aTra::v0.11.pre
   3:     0x7fde21141f60 - rt::unwind::begin_unwind::ha806566b31ba8340qXp::v0.11.pre
   4:     0x7fde21538520 - middle::check_match::is_useful_specialized::h5bccec681de6c692gAu::v0.11.pre
   5:     0x7fde21530f40 - middle::check_match::is_useful::h5ffe9bbdddc6b96fWtu::v0.11.pre
   6:     0x7fde2152a770 - middle::check_match::check_expr::hd8afce200b8873afQau::v0.11.pre
   7:     0x7fde215303d0 - visit::walk_expr_opt::h85f69636933670ccMku::v0.11.pre
   8:     0x7fde2152d160 - middle::check_match::check_fn::h67f8bdc6ca535dcbbdv::v0.11.pre
   9:     0x7fde2152de10 - visit::walk_item::hd4abe3032d44eb0fQjw::v0.11.pre
  10:     0x7fde2152d3e0 - middle::check_match::check_crate::h945724cd4dbeabc2bau::v0.11.pre
  11:     0x7fde21525c70 - util::common::time::h8ad21ae303e4a453Swg::v0.11.pre
  12:     0x7fde21963e80 - driver::driver::phase_3_run_analysis_passes::h679c7da862d3eab5sje::v0.11.pre
  13:     0x7fde2196a890 - driver::driver::compile_input::h78d619679e701e77nIe::v0.11.pre
  14:     0x7fde2198edf0 - run_compiler::h304797548f90c81exgm::v0.11.pre
  15:     0x7fde219a64f0 - main_args::closure.91891
  16:     0x7fde219a4e20 - monitor::closure.91776
  17:     0x7fde219a0700 - task::TaskBuilder::try::closure.91542
  18:     0x7fde20c1aab0 - task::spawn_opts::closure.7971
  19:     0x7fde205846d0 - rt::task::Task::run::closure.40115
  20:     0x7fde20590230 - rust_try
  21:     0x7fde20584510 - rt::task::Task::run::h5360bb699175a8aefi8::v0.11.pre
  22:     0x7fde20c1a880 - task::spawn_opts::closure.7944
  23:     0x7fde205881e0 - rt::thread::thread_start::h8ec2412a6038d9b80W8::v0.11.pre
  24:     0x7fde1ddbdfa0 - start_thread
  25:     0x7fde201bea09 - __clone
  26:                0x0 - <unknown>
