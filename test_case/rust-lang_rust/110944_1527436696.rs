plain
    |
562 |         ecx: &InterpCx<'mir, 'tcx, Self>,
    |         ^^^ help: if this is intentional, prefix it with an underscore: `_ecx`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `bin_op`
   --> compiler/rustc_const_eval/src/const_eval/machine.rs:563:9
    |
563 |         bin_op: mir::BinOp,
