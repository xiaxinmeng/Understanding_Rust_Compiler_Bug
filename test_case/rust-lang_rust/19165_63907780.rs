
RUST_BACKTRACE=1 cargo build --verbose
       Fresh encoding_index_tests v0.1.0 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh gcc v0.0.1 (https://github.com/alexcrichton/gcc-rs#f25b3ba9)
       Fresh encoding-index-tradchinese v1.0.20140915 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh encoding-index-korean v1.0.20140915 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh encoding-index-simpchinese v1.0.20140915 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh encoding-index-japanese v1.0.20140915 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh encoding-index-singlebyte v1.0.20140915 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh encoding v0.2.3 (http://github.com/lifthrasiir/rust-encoding#b1eee875)
       Fresh url v0.1.0 (https://github.com/servo/rust-url#5d15bee2)
       Fresh time v0.0.3 (https://github.com/rust-lang/time#9595d0bc)
   Compiling tiny_http v0.0.1 (file:///home/flubba86/ClionProjects/tiny-http(upload))
     Running `rustc /home/flubba86/ClionProjects/tiny-http(upload)/src/lib.rs --crate-name tiny_http --crate-type lib -g -C metadata=a4527134254ec2f4 -C extra-filename=-a4527134254ec2f4 --out-dir /home/flubba86/ClionProjects/tiny-http(upload)/target --dep-info /home/flubba86/ClionProjects/tiny-http(upload)/target/.fingerprint/tiny_http-a4527134254ec2f4/dep-lib-tiny_http -L /home/flubba86/ClionProjects/tiny-http(upload)/target -L /home/flubba86/ClionProjects/tiny-http(upload)/target/deps --extern encoding=/home/flubba86/ClionProjects/tiny-http(upload)/target/deps/libencoding-83b8e0a6e831c6fe.rlib --extern url=/home/flubba86/ClionProjects/tiny-http(upload)/target/deps/liburl-921578b148f50e06.rlib --extern time=/home/flubba86/ClionProjects/tiny-http(upload)/target/deps/libtime-d924131696e54b12.rlib -L /home/flubba86/ClionProjects/tiny-http(upload)/target/build/time-d924131696e54b12/out`
<std macros>:3:24: 74:23 error: type `W` cannot be dereferenced
<std macros>:3         let dst = &mut *$dst;
<std macros>:4         format_args!(|args| { dst.write_fmt(args) }, $($arg)*)
<std macros>:5     })
<std macros>:6 )
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'index out of bounds: the len is 6 but the index is 6', /home/flubba86/ClionProjects/rust1/src/libsyntax/lib.rs:1

stack backtrace:
   1:     0x7f5ee82db4b0 - rt::backtrace::imp::write::h8a405cfd8c4396c8B9s
   2:     0x7f5ee82ddf80 - failure::on_fail::h25e4014ba8bd0150uwt
   3:     0x7f5ee9030590 - unwind::begin_unwind_inner::h036f42082d648fc6nbd
   4:     0x7f5ee902fed0 - unwind::begin_unwind_fmt::h63d6194988bc700ey8c
   5:     0x7f5ee902fe60 - unwind::rust_begin_unwind::__rust_abi
   6:     0x7f5ee902fe00 - rust_begin_unwind
   7:     0x7f5ee90b6600 - panicking::panic_fmt::h9b64d3d148d43babaOl
   8:     0x7f5ee90ca8f0 - panicking::panic_bounds_check::closure.24524
   9:     0x7f5ee90ca230 - panicking::panic_bounds_check::h8b09047c8e0d6de5IMl
  10:     0x7f5ee4dce220 - vec::Vec<T>.Index<uint, T>::index::h13296517075474870301
  11:     0x7f5ee4dce7d0 - codemap::FileMap::get_line::h968503406ea5c987m4E
  12:     0x7f5ee4e29a60 - diagnostic::highlight_lines::he212720ca7df7227mAG
  13:     0x7f5ee4e27320 - diagnostic::emit::h96f49ef21ff994c3wvG
  14:     0x7f5ee4e1e2a0 - diagnostic::EmitterWriter.Emitter::emit::hfc2b401a44e9fb57lrG
  15:     0x7f5ee4e1b170 - diagnostic::Handler::emit::h2c6717da6b5795acF8F
  16:     0x7f5ee4dc05b0 - diagnostic::SpanHandler::span_err::h2cae115528589b17EWF
  17:     0x7f5ee62db1c0 - session::Session::span_err::h1290354ba20547c8Rq1
  18:     0x7f5ee6dbefc0 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message_str_with_expected::h0bb8f60cea19de07TKF
  19:     0x7f5ee6dbee90 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message_str::hb7cef490efc13ee4aKF
  20:     0x7f5ee6b8d460 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message::h2d3cc777e752438bdQF
  21:     0x7f5ee6a01df0 - middle::typeck::check::FnCtxt<'a, 'tcx>::type_error_message::ha6fb48c500670cb9jdl
  22:     0x7f5ee6bc2b10 - middle::typeck::check::check_expr_with_unifier::h0dbf737eb021cff9Nzm
  23:     0x7f5ee6bc9050 - middle::typeck::check::check_expr_with_expectation_and_lvalue_pref::h86dfcb5ca588d4a0Erm
  24:     0x7f5ee6bc2b10 - middle::typeck::check::check_expr_with_unifier::h0dbf737eb021cff9Nzm
  25:     0x7f5ee6bc1540 - middle::typeck::check::check_expr_coercable_to_type::h31b8b00ee254719cSpm
  26:     0x7f5ee6c2fe40 - middle::typeck::check::check_decl_initializer::h5bc40a7eb0aae7a9Kso
  27:     0x7f5ee6c2fec0 - middle::typeck::check::check_decl_local::h855c07b06be28625gto
  28:     0x7f5ee6c30200 - middle::typeck::check::check_stmt::h5ef35cdc4d9c47f5gvo
  29:     0x7f5ee6afe8d0 - middle::typeck::check::check_block_with_expected::hc777cdebc78f5772vzo
  30:     0x7f5ee6bc2b10 - middle::typeck::check::check_expr_with_unifier::h0dbf737eb021cff9Nzm
  31:     0x7f5ee68a5e20 - middle::typeck::check::check_expr_has_type::hdfc9e782e51d827dapm
  32:     0x7f5ee68a5ac0 - middle::typeck::check::_match::check_match::hfecd5ed6941017b05P8
  33:     0x7f5ee6bc2b10 - middle::typeck::check::check_expr_with_unifier::h0dbf737eb021cff9Nzm
  34:     0x7f5ee6b279f0 - middle::typeck::check::check_expr_with_expectation::hed9aa97cedf54cf27qm
  35:     0x7f5ee6afe8d0 - middle::typeck::check::check_block_with_expected::hc777cdebc78f5772vzo
  36:     0x7f5ee6bc2b10 - middle::typeck::check::check_expr_with_unifier::h0dbf737eb021cff9Nzm
  37:     0x7f5ee689f320 - middle::typeck::check::check_expr::hf8316221db8c619cfsm
  38:     0x7f5ee6c30200 - middle::typeck::check::check_stmt::h5ef35cdc4d9c47f5gvo
  39:     0x7f5ee6afe8d0 - middle::typeck::check::check_block_with_expected::hc777cdebc78f5772vzo
  40:     0x7f5ee6a8d8e0 - middle::typeck::check::check_fn::h04a0d7b2ed09a1cce0i
  41:     0x7f5ee6ae3770 - middle::typeck::check::check_bare_fn::h523bbaa22a732feejPi
  42:     0x7f5ee6adaee0 - middle::typeck::check::check_item::hd0b9ae388150a0a0t9i
  43:     0x7f5ee6adae80 - middle::typeck::check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'v>::visit_item::h4b78b00102611fa2WNi
  44:     0x7f5ee6ae1c60 - visit::walk_mod::h17267871328085943818
  45:     0x7f5ee6ae1c00 - visit::Visitor::visit_mod::h3243897094775644115
  46:     0x7f5ee6adc5d0 - visit::walk_item::h10656758597348061101
  47:     0x7f5ee6adae80 - middle::typeck::check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'v>::visit_item::h4b78b00102611fa2WNi
  48:     0x7f5ee6ae1c60 - visit::walk_mod::h17267871328085943818
  49:     0x7f5ee6ae1c00 - visit::Visitor::visit_mod::h3243897094775644115
  50:     0x7f5ee6ae3690 - visit::walk_crate::h17586396567340744879
  51:     0x7f5ee6ae2ed0 - middle::typeck::check::check_item_types::hed950dc88cf71222tOi
  52:     0x7f5ee72f1ad0 - middle::typeck::check_crate::closure.142588
  53:     0x7f5ee72ee9d0 - util::common::time::h10144082460099455207
  54:     0x7f5ee72ee6d0 - middle::typeck::check_crate::h7f14887da1f59c09cwM
  55:     0x7f5ee99267a0 - driver::driver::phase_3_run_analysis_passes::h8814e32378c6f9batfS
  56:     0x7f5ee991f860 - driver::driver::compile_input::hcd47e1ad54a22e70dWR
  57:     0x7f5ee9a539b0 - driver::run_compiler::hedc32aa9be3cae57HUT
  58:     0x7f5ee9a53870 - driver::run::closure.65110
  59:     0x7f5ee9486a00 - task::TaskBuilder<S>::try_future::closure.39355
  60:     0x7f5ee94864e0 - task::TaskBuilder<S>::spawn_internal::closure.39324
  61:     0x7f5ee9e9cd30 - task::NativeSpawner.Spawner::spawn::closure.2472
  62:     0x7f5ee902f7a0 - unwind::try::try_fn::__rust_abi
  63:     0x7f5ee902f300 - unwind::try::try_fn::haf1738b98ae171e8V1c
  64:     0x7f5ee90f2930 - rust_try_inner
  65:     0x7f5ee90f2920 - rust_try
  66:     0x7f5ee902a010 - unwind::try::h036278122fffae79IZc
  67:     0x7f5ee9029cf0 - task::Task::run::h99a74865c94ccad2y5b
  68:     0x7f5ee9e9c7f0 - task::NativeSpawner.Spawner::spawn::closure.2396
  69:     0x7f5ee902d7e0 - thread::start_thread::h01c702f00a03d242Spc
  70:     0x7f5ee902da10 - thread::thread_start::__rust_abi
  71:     0x7f5ee902d9f0 - thread::thread_start::h59f79b652abf5cd3Eqc
  72:     0x7f5ee89ecfe0 - start_thread
  73:     0x7f5ee8ce7c89 - __clone
  74:                0x0 - <unknown>

Could not compile `tiny_http`.

Caused by:
  Process didn't exit successfully: `rustc /home/flubba86/ClionProjects/tiny-http(upload)/src/lib.rs --crate-name tiny_http --crate-type lib -g -C metadata=a4527134254ec2f4 -C extra-filename=-a4527134254ec2f4 --out-dir /home/flubba86/ClionProjects/tiny-http(upload)/target --dep-info /home/flubba86/ClionProjects/tiny-http(upload)/target/.fingerprint/tiny_http-a4527134254ec2f4/dep-lib-tiny_http -L /home/flubba86/ClionProjects/tiny-http(upload)/target -L /home/flubba86/ClionProjects/tiny-http(upload)/target/deps --extern encoding=/home/flubba86/ClionProjects/tiny-http(upload)/target/deps/libencoding-83b8e0a6e831c6fe.rlib --extern url=/home/flubba86/ClionProjects/tiny-http(upload)/target/deps/liburl-921578b148f50e06.rlib --extern time=/home/flubba86/ClionProjects/tiny-http(upload)/target/deps/libtime-d924131696e54b12.rlib -L /home/flubba86/ClionProjects/tiny-http(upload)/target/build/time-d924131696e54b12/out` (status=101)
