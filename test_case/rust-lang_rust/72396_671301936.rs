rust
enum NotSureWhatToCallThis<T> {
  Unevaluated(...),
  Param(...),
  Error(...),
  Evaluated(T)
}

mod ty { type Const = NotSureWhatToCallThis<Valtree>; }
mod mir { type Const = NotSureWhatToCallThis<&'tcx Allocation>; }
