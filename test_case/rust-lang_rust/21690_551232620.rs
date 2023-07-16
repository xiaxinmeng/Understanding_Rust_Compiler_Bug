
#[math(overflow="wrap", assumptions=("algebraic", "no-nan", "finite"))]
fn update_velocity(b1: &mut Body, b2: &mut Body, diff: f32, mag: f32) {
    b1.vel = b1.vel - diff * (b2.mass * mag);
    b2.vel = b2.vel + diff * (b1.mass * mag);
}
