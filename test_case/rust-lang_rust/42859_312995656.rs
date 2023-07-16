
[00:10:31] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:10:31]    --> /checkout/src/librustc_const_eval/eval.rs:344:34
[00:10:31]     |
[00:10:31] 344 |           if tcx.type_of(def_id).fn_sig().abi() == Abi::RustIntrinsic {
[00:10:31]     |                                  ^^^^^^ expected 1 parameter
[00:10:31]
[00:10:32] error: aborting due to previous error(s)
[00:10:32]
[00:10:32] error: Could not compile `rustc_const_eval`.
