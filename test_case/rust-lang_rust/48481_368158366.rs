rust
let x: &dyn (Debug + Send) = ..; // this should work, right?
let x: &dyn (Debug + Send + Sync) = ..; // this too
let x: &dyn (Debug + Send + ) = ..; // I think we support trailing `+` terminators elsewhere
let x: Box<dyn (Debug + Send) + Sync> = ...; // what about this? =) I forget what we decided there
