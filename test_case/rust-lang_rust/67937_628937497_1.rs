rust
let distance = target_x.saturating_sub(self.x());
// Ceiling division.
((distance + Self::max_speed() as u32 - 1) / Self::max_speed() as u32) as u16
