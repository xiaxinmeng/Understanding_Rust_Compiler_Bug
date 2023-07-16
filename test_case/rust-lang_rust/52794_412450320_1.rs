rust
fn dispatch<Op: Operation>(&self, op: &Op) -> Op::Response
