rust
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust 
$ grep -nrIFw CheckedBinaryOp
src/librustc/ich/impls_mir.rs:347:            mir::Rvalue::CheckedBinaryOp(op, ref operand1, ref operand2) => {
src/librustc/mir/tcx.rs:238:            Rvalue::CheckedBinaryOp(op, ref lhs, ref rhs) => {
src/librustc/mir/mod.rs:2191:    CheckedBinaryOp(BinOp, Operand<'tcx>, Operand<'tcx>),
src/librustc/mir/mod.rs:2330:            CheckedBinaryOp(ref op, ref a, ref b) => {
src/librustc/mir/mod.rs:3318:            CheckedBinaryOp(op, ref rhs, ref lhs) => {
src/librustc/mir/mod.rs:3319:                CheckedBinaryOp(op, rhs.fold_with(folder), lhs.fold_with(folder))
src/librustc/mir/mod.rs:3355:            BinaryOp(_, ref rhs, ref lhs) | CheckedBinaryOp(_, ref rhs, ref lhs) => {
src/librustc/mir/visit.rs:626:                    Rvalue::CheckedBinaryOp(_bin_op,
src/librustc_mir/borrow_check/nll/type_check/mod.rs:2005:            | Rvalue::CheckedBinaryOp(..)
src/librustc_mir/borrow_check/nll/type_check/mod.rs:2022:            | Rvalue::CheckedBinaryOp(..)
src/librustc_mir/borrow_check/nll/invalidation.rs:371:            | Rvalue::CheckedBinaryOp(_bin_op, ref operand1, ref operand2) => {
src/librustc_mir/borrow_check/mod.rs:1206:            | Rvalue::CheckedBinaryOp(_bin_op, ref operand1, ref operand2) => {
src/librustc_mir/build/expr/as_rvalue.rs:416:                Rvalue::CheckedBinaryOp(op, lhs, rhs),
src/librustc_mir/dataflow/move_paths/builder.rs:309:            Rvalue::CheckedBinaryOp(ref _binop, ref lhs, ref rhs) => {
src/librustc_mir/interpret/step.rs:163:            CheckedBinaryOp(bin_op, ref left, ref right) => {
src/librustc_mir/transform/lower_128bit.rs:75:                            | Rvalue::CheckedBinaryOp(_, lhs, rhs) => (place, lhs, rhs),
src/librustc_mir/transform/lower_128bit.rs:157:        StatementKind::Assign(_, box Rvalue::CheckedBinaryOp(bin_op, ref lhs, _)) => {
src/librustc_mir/transform/const_prop.rs:399:            Rvalue::CheckedBinaryOp(op, ref left, ref right) |
src/librustc_mir/transform/const_prop.rs:454:                let val = if let Rvalue::CheckedBinaryOp(..) = *rvalue {
src/librustc_mir/transform/qualify_consts.rs:586:            Rvalue::CheckedBinaryOp(..) |
src/librustc_mir/transform/qualify_min_const_fn.rs:172:        Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
src/librustc_codegen_ssa/mir/rvalue.rs:435:            mir::Rvalue::CheckedBinaryOp(op, ref lhs, ref rhs) => {
src/librustc_codegen_ssa/mir/rvalue.rs:716:            mir::Rvalue::CheckedBinaryOp(..) |
