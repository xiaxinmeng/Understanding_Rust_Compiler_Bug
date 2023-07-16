
$ env RUST_BACKTRACE=1 rustc commands.rs
commands.rs:17:20: 17:25 error: mismatched types: expected `uint` but found `int` (expected uint but found int)
commands.rs:17      match tokens.get(index) {
                                     ^~~~~
commands.rs:18:4: 18:13 error: mismatched types: expected `std::option::Option<&&str>` but found `std::result::Result<<generic #17>,<generic #18>>` (expected enum std::option::Option but found enum std::result::Result)
commands.rs:18          Ok(value) => int::parse_bytes(value.as_bytes(), 10),
                        ^~~~~~~~~
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 0 but the index is 0', /private/tmp/rust-Q8uz/rust-0.10/src/librustc/lib.rs:1
stack backtrace:
   1:        0x107cb28e4 - rt::backtrace::imp::write::h2a8a3b7117b6a3bag8b::v0.10
   2:        0x107c16b56 - rt::unwind::begin_unwind_inner::h13ee063055871ef1KIb::v0.10
   3:        0x107c15fc8 - rt::unwind::begin_unwind::hcf46aa8743077363kIb::v0.10
   4:        0x107cb23b9 - rt::unwind::begin_unwind_raw::h9f0f430a7522466dTFb::v0.10
   5:        0x107c1523e - rt::unwind::fail_::hd92116ac6b4e151axDb::v0.10
   6:        0x107cb2402 - rt::unwind::fail_bounds_check::closure.41732
   7:        0x107c172ce - rt::unwind::fail_bounds_check::h608f2f2d28d48d89UDb::v0.10
   8:        0x1055f23ec - middle::typeck::check::method::LookupContext$LT$$x27a$GT$::push_bound_candidates::closure.66574
   9:        0x1055f0f01 - middle::typeck::check::autoderef::hf4fdb94b1d06a293Oz9::v0.10
  10:        0x1055edf93 - middle::typeck::check::method::LookupContext$LT$$x27a$GT$::push_bound_candidates::h4eebbd30dc096d68cf6::v0.10
  11:        0x1055ee1f0 - middle::typeck::check::method::lookup_in_trait::h4fe82a71d0abe009ZX5::v0.10
  12:        0x1055eec39 - middle::typeck::check::try_overloaded_deref::h9be89e6dcd449c3aKD9::v0.10
  13:        0x1055f1044 - middle::typeck::check::autoderef::hf4fdb94b1d06a293Oz9::v0.10
  14:        0x1055ec9f0 - middle::typeck::check::method::lookup::h91adc5fe6eb35f8b3Q5::v0.10
  15:        0x105627335 - middle::typeck::check::check_expr_with_unifier::h9611897cf402e0e1X79::v0.10
  16:        0x10563042c - middle::typeck::check::check_expr_with_unifier::check_argument_types::h91f9c529b5bd0134Zba::v0.10
  17:        0x10562b0a8 - middle::typeck::check::check_expr_with_unifier::h9611897cf402e0e1X79::v0.10
  18:        0x10559353d - middle::typeck::check::_match::check_match::hdddd2803abbbbf71Oe0::v0.10
  19:        0x105627c5e - middle::typeck::check::check_expr_with_unifier::h9611897cf402e0e1X79::v0.10
  20:        0x105606c39 - middle::typeck::check::check_block_with_expected::h2c7bb960e076eb42Rrc::v0.10
  21:        0x10560275d - middle::typeck::check::check_fn::h1c2cafb90241ad96p67::v0.10
  22:        0x105601f42 - middle::typeck::check::check_bare_fn::hf0b73f6491c4bb03hW7::v0.10
  23:        0x10560b6fb - middle::typeck::check::check_method_body::h49c9b749692cc863kr8::v0.10
  24:        0x1055fdaf6 - middle::typeck::check::check_item::hb8facf87dab582d4Pg8::v0.10
  25:        0x105601dbe - middle::typeck::check::check_item_types::hdb126557beb409a1RV7::v0.10
  26:        0x10573013f - util::common::time::h9acad76c21817c9aXHh::v0.10
  27:        0x10572f049 - middle::typeck::check_crate::hf3137d246c3f533atTu::v0.10
  28:        0x105b3f775 - driver::driver::phase_3_run_analysis_passes::h605fda952e28c418vxf::v0.10
  29:        0x105b45a63 - driver::driver::compile_input::haa7c9f4729d6243csWf::v0.10
  30:        0x105b6b6c4 - run_compiler::hfd5ecc9785222053Yzn::v0.10
  31:        0x105b7f36d - main_args::closure.92066
  32:        0x105b7db82 - monitor::closure.91951
  33:        0x105b7972b - task::TaskBuilder::try::closure.91726
  34:        0x10533d49c - task::spawn_opts::closure.7957
  35:        0x107cadd08 - rt::task::Task::run::closure.41637
  36:        0x107cb85fc - rust_try
  37:        0x107cadb87 - rt::task::Task::run::h544c274b867f177dNB9::v0.10
  38:        0x10533d31f - task::spawn_opts::closure.7929
  39:        0x107cb12c6 - rt::thread::thread_start::hc06c53d926f6e6a3uga::v0.10
  40:     0x7fff90a76899 - _pthread_body
  41:     0x7fff90a7672a - _pthread_struct_init
