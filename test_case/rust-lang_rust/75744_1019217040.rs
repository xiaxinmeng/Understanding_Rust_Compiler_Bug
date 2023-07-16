rust
trait BoolControlFlowExt {
  fn break(self) -> ControlFlow;
  fn continue(self) -> ControFlow;
}

impl BoolControlFlowExt for bool {
  fn break(self) -> ControlFlow {
    match self {
          true => ControlFlow::Break,
          false => ControlFlow::Continue,
}

  fn continue(self) -> ControlFlow {
    match self {
          true => ControlFlow::Continue,
          false => ControlFlow::Break,
}
  }
}

// Some code...
....
my_condition.break()?;
....
