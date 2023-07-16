rust
use bevy::ecs::query::QueryItem;

fn main() {}

fn first_child() -> impl FnOnce(&u32) -> Result<QueryItem<u32>, u32> {}
