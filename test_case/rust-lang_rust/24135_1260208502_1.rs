
pub struct Child {
  id: u32
  state: ChildState,
}

enum ChildState {
  Running(Handle),
  Exited(std::io::Result<ExitStatus>),
}
