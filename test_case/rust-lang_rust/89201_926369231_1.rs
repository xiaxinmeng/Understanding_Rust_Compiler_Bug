
error[E0599]: no function or associated item named `of_instance` found for struct `FnAbi` in the current scope
  --> src/base.rs:65:29
   |
65 |         fn_abi: Some(FnAbi::of_instance(&RevealAllLayoutCx(tcx), instance, &[])),
   |                             ^^^^^^^^^^^ function or associated item not found in `FnAbi<'_, _>

