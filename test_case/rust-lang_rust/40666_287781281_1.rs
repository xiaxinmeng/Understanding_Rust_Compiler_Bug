rust
struct Character { money: f64, .... }

enum Entity {
    Character(Character),
    ....
}

match self {
    Entity::Character(ref mut ch) { ch.money = .... }
}
