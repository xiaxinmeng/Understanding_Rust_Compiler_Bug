
fn set_noncopyable_2(x: thing) -> builder {
    assert !self.consumed;
    self.consumed = true;
    let mut tmp = none;
    self.noncopyable_1 <-> tmp;
    { consumed: false, noncopyable_1: tmp, noncopyable_2: x with self }
}
