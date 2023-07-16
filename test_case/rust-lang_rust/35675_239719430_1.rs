 rust
use Fruit::*;
enum Fruit {
    Apple(i64),
    Orange(i64),
}

fn should_return_fruit() -> Apple {
    Apple(5)
}
