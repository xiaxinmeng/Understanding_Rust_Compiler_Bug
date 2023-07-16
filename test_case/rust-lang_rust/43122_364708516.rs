
fn resume(&mut self) -> Option<Self::Yield>;
fn await_done(self) -> Self::Return;
