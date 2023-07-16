rust
enum MirConstValue {
  Literal(Valtree),
  Alloc(&'tcx Allocation),
}

mod ty { type Const = NotSureWhatToCallThis<Valtree>; }
mod mir { type Const = NotSureWhatToCallThis<MirConstValue>; }
