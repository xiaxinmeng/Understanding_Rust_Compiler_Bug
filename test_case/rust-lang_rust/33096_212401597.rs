
extern crate nphysics3d;

pub fn main() {
    nphysics3d::world::World::new().step(1.0);
}
