
#[macro_use]
extern crate slotmap;

use slotmap::SlotMap;

new_key_type! {
    struct EntityKey;

    /// Key for the Player slot map.
    pub struct PlayerKey;
}

fn main() {
    let mut players = SlotMap::with_key();
    let mut entities: SlotMap<EntityKey, (f64, f64)> = SlotMap::with_key();
    let bob: PlayerKey = players.insert("bobby");
    // Now this is a type error because entities.get expects an EntityKey:
    // entities.get(bob);
}
