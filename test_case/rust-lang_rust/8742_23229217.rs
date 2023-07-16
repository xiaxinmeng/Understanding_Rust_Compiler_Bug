
fn reserve_space_for_more(&mut self, more: uint) {  // find me a better name and write me less silly
    let newcap = self.len().checked_add(&more).expect("reserve_space_for_more: Overflow");
    self.reserve(cmp::max(newcap, uint::next_power_of_two(newcap))
}
