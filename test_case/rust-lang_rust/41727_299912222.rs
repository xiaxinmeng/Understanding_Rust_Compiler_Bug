rust
fn main() {
    let mut collision = None;
    collision = collision.map_or(Some(22_u32), |c| Some(c.saturating_add(1)));
}
