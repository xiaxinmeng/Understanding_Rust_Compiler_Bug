plain
    Checking clippy_lints v0.1.69 (/checkout/src/tools/clippy/clippy_lints)
error[E0425]: cannot find value `from_fn` in module `sym`
  --> src/tools/clippy/clippy_lints/src/operators/cmp_owned.rs:55:58
   |
55 |                 } else if cx.tcx.is_diagnostic_item(sym::from_fn, did) {
   |                                                          ^^^^^^^ not found in `sym`
help: consider importing one of these items
   |
1  | use core::array::from_fn;
   |
---
   |
     and 1 other candidate
help: if you import `from_fn`, refer to it directly
   |
55 -                 } else if cx.tcx.is_diagnostic_item(sym::from_fn, did) {
55 +                 } else if cx.tcx.is_diagnostic_item(from_fn, did) {

error[E0425]: cannot find value `from_fn` in module `sym`
  --> src/tools/clippy/clippy_lints/src/unnecessary_owned_empty_strings.rs:58:59
   |
   |
58 |                         if cx.tcx.is_diagnostic_item(sym::from_fn, fun_def_id);
   |                                                           ^^^^^^^ not found in `sym`
help: consider importing one of these items
   |
1  | use core::array::from_fn;
   |
---
   |
     and 1 other candidate
help: if you import `from_fn`, refer to it directly
   |
58 -                         if cx.tcx.is_diagnostic_item(sym::from_fn, fun_def_id);
58 +                         if cx.tcx.is_diagnostic_item(from_fn, fun_def_id);

error[E0425]: cannot find value `from_fn` in module `sym`
   --> src/tools/clippy/clippy_lints/src/useless_conversion.rs:164:63
    |
    |
164 | ...                   if cx.tcx.is_diagnostic_item(sym::from_fn, def_id);
    |                                                         ^^^^^^^ not found in `sym`
help: consider importing one of these items
    |
1   | use core::array::from_fn;
    |
---
    |
      and 1 other candidate
help: if you import `from_fn`, refer to it directly
    |
164 -                             if cx.tcx.is_diagnostic_item(sym::from_fn, def_id);
164 +                             if cx.tcx.is_diagnostic_item(from_fn, def_id);

    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
For more information about this error, try `rustc --explain E0425`.
error: could not compile `clippy_lints` due to 3 previous errors
