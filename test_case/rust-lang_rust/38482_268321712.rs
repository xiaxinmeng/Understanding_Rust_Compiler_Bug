
error[E0053]: method `to_ret` has an incompatible type for trait
   --> /checkout/src/libcompiler_builtins/lib.rs:505:28
    |
426 |         fn to_ret(self) -> Self::Ret;
    |                            --------- type in trait
...
505 |         fn to_ret(self) -> u128ret {
    |                            ^^^^^^^ expected i128, found u128
error: aborting due to previous error
