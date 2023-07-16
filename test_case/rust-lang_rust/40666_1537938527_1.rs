rs
pub fn modify(&mut self) {
    match self {
        Entity::Character {ref mut money, ..} => {
            money = ...
