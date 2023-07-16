rust
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter, Serialize)]
pub enum Register<T: Default> {
  RAX,
  RBX,
  RCX,
  RDX,
  RDI,
  RSI,
  RBP,
  RSP,
  R8,
  R9,
  R10,
  R11,
  R12,
  R13,
  R14,
  R15,
  _Unreachable(PhantomData<T>),
}
