 rust
pub struct MovePathIndex(usize);

const INVALID_MOVE_PATH_INDEX: MovePathIndex = MovePathIndex(usize::MAX);

impl Idx for MovePathIndex {
    type Data = MovePath;
    const INVALID: Self = INVALID_MOVE_PATH_INDEX;
    fn idx(&self) -> usize { self.0 }
}
