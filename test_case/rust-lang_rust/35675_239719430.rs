 rust
enum Fruit {
    Apple(i64),
    Orange(i64),
}

fn should_return_fruit() -> Fruit::Apple {
    Fruit::Apple(5)
}
