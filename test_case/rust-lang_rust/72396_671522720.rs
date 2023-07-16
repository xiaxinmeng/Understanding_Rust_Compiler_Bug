rust
//! The evaluated result of a constant in the MIR.
enum MirConstValue {
  Scalar(...),
  ScalarPair(...), // or "Slice" or so -- I suppose we cannot entirely avoid this
  Alloc(&'tcx Allocation),
}

mod ty { type Const = NotSureWhatToCallThis<Valtree>; }
mod mir { type Const = NotSureWhatToCallThis<MirConstValue>; }
