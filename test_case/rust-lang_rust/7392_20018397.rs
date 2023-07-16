
fn read_unless_poisoned(blk: fn(Option<&T>) -> U) -> U
fn write_unless_poisoned(blk: fn(Option<&mut T>) -> U) -> U
fn write_cond_unless_poisoned(blk: fn(Option<(&mut T, &Condvar)>) -> U) -> U
fn write_downgrade_unless_poisoned(blk: fn(Option<RWWriteMode>) -> U) -> U
