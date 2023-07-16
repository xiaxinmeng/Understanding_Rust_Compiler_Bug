
$ RUST_BACKTRACE=1 cargo build
   Compiling datetime v0.1.7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'index out of bounds: the len is 60 but the index is 117', /private/tmp/rust20150518-72266-n7gtuc/src/libcollections/vec.rs:1361

stack backtrace:
   1:        0x10fe31aef - sys::backtrace::write::ha44b3f4df72d371c32r
   2:        0x10fe3a5d0 - panicking::on_panic::h19b50da5ca5e05bauiw
   3:        0x10fdf5945 - rt::unwind::begin_unwind_inner::h0b7205538a0ab840d0v
   4:        0x10fdf674c - rt::unwind::begin_unwind_fmt::h88d3334f8dd63530jZv
   5:        0x10fe3a12c - rust_begin_unwind
   6:        0x10fe8c815 - panicking::panic_fmt::h41ecfe1a10526c59r3B
   7:        0x10fe876a8 - panicking::panic_bounds_check::h1e29ed4cc92cd76ax2B
   8:        0x116649805 - parse::token::get_name::h35e7e5fccea6e44dnZT
   9:        0x116742ef4 - parse::parser::Parser<'a>::lit_from_token::hf58c8962037cb225ZzH
  10:        0x116747f47 - parse::parser::Parser<'a>::parse_lit::h2e50eafa1ebd61ddZDH
  11:        0x116750023 - parse::parser::Parser<'a>::parse_bottom_expr::h9e9a850fefa543e4i9H
  12:        0x116755682 - parse::parser::Parser<'a>::parse_dot_or_call_expr::h05b5aa9a18a416d2iCI
  13:        0x116759c99 - parse::parser::Parser<'a>::parse_prefix_expr::hcdeb32d1451049bax5I
  14:        0x11675a412 - parse::parser::Parser<'a>::parse_binops::he3e0fd996016893ekeJ
  15:        0x11675aee5 - parse::parser::Parser<'a>::parse_assign_expr::h53209e75bf3e1361ckJ
  16:        0x116723861 - parse::parser::Parser<'a>::parse_expr::h9c3b3a2e323926e41RF
  17:        0x11155937e - parse::h2eeaa28acf5ad6604Se
  18:        0x111558b5e - native::h055b9d86463f67cc2aa
  19:        0x114856c43 - ext::base::F.TTMacroExpander::expand::h16589758004100754197
  20:        0x10f75aed8 - ext::expand::expand_expr::closure.66525
  21:        0x10f7586c0 - ext::expand::expand_expr::h205e82f220007a1f9Lb
  22:        0x10f761470 - fold::noop_fold_expr::h15685179544264607782
  23:        0x10f758d1b - ext::expand::expand_expr::closure.66525
  24:        0x10f7586c0 - ext::expand::expand_expr::h205e82f220007a1f9Lb
  25:        0x10f761cb4 - fold::noop_fold_expr::h15685179544264607782
  26:        0x10f758d1b - ext::expand::expand_expr::closure.66525
  27:        0x10f7586c0 - ext::expand::expand_expr::h205e82f220007a1f9Lb
  28:        0x10f7a6557 - ext::expand::expand_block_elts::closure.67187
  29:        0x10f76c359 - ext::expand::expand_block_elts::h1415cb18ccd320f2Mxc
  30:        0x10f7a6172 - ext::expand::expand_block::h7d5c060d19c0744c7wc
  31:        0x10f76bdc1 - ext::expand::expand_and_rename_fn_decl_and_block::h4a7a8fe955c244cab1c
  32:        0x10f7724aa - ext::expand::expand_item_underscore::h264c173708ffb055hbc
  33:        0x10f7c611c - fold::noop_fold_item_simple::h15088194635162742853
  34:        0x10f7c5efe - ptr::P<T>::map::h2926393362253440021
  35:        0x10f770491 - ext::expand::expand_annotatable::hbcf3d8c020636cdb8Hc
  36:        0x10f76c4d9 - ext::expand::expand_item::h4a790b6e8b59cdb7Gac
  37:        0x10f777602 - iter::FlatMap<I, U, F>.Iterator::next::h7298186205452440470
  38:        0x10f777235 - vec::Vec<T>.FromIterator<T>::from_iter::h2720693747523327938
  39:        0x10f772cf6 - ext::expand::expand_item_underscore::h264c173708ffb055hbc
  40:        0x10f7c611c - fold::noop_fold_item_simple::h15088194635162742853
  41:        0x10f7c5efe - ptr::P<T>::map::h2926393362253440021
  42:        0x10f771487 - ext::expand::expand_annotatable::hbcf3d8c020636cdb8Hc
  43:        0x10f76c4d9 - ext::expand::expand_item::h4a790b6e8b59cdb7Gac
  44:        0x10f777602 - iter::FlatMap<I, U, F>.Iterator::next::h7298186205452440470
  45:        0x10f777235 - vec::Vec<T>.FromIterator<T>::from_iter::h2720693747523327938
  46:        0x10f772cf6 - ext::expand::expand_item_underscore::h264c173708ffb055hbc
  47:        0x10f7c611c - fold::noop_fold_item_simple::h15088194635162742853
  48:        0x10f7c5efe - ptr::P<T>::map::h2926393362253440021
  49:        0x10f771487 - ext::expand::expand_annotatable::hbcf3d8c020636cdb8Hc
  50:        0x10f76c4d9 - ext::expand::expand_item::h4a790b6e8b59cdb7Gac
  51:        0x10f7cda05 - ext::expand::expand_crate::hb3a4d686dfcac529E9c
  52:        0x10c91d5d5 - driver::phase_2_configure_and_expand::closure.20416
  53:        0x10c8d00ae - driver::phase_2_configure_and_expand::h6e78b7cbbae1e1ccYsa
  54:        0x10c8c0369 - driver::compile_input::hd0ddbf75d3cab180Qba
  55:        0x10c97cf63 - run_compiler::hc7fc80763a6a7ff875b
  56:        0x10c97a6ca - boxed::F.FnBox<A>::call_box::h10164488635330116687
  57:        0x10c979c27 - rt::unwind::try::try_fn::h2426084354629403706
  58:        0x10fec0458 - rust_try_inner
  59:        0x10fec0445 - rust_try
  60:        0x10c979efd - boxed::F.FnBox<A>::call_box::h15095568843964241812
  61:        0x10fe391bd - sys::thread::Thread::new::thread_start::h6ac7556a78cca2c6Hlv
  62:     0x7fff94422898 - _pthread_body
  63:     0x7fff94422729 - _pthread_start
