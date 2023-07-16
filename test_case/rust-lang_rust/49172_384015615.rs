rust
// ok
const X: FakeNeedsDrop = { let x = FakeNeedsDrop; x };
// error
const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
// error
const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); }
