rust
union Foo { bar: i8, pizza: Pizza }
#[derive(Copy, Clone)]
struct Pizza {
    topping: Option<PizzaTopping>
}
#[derive(Copy, Clone)]
enum PizzaTopping { Cheese, Pineapple }

let foo = Foo { bar: 5 };
match foo { Foo {
    pizza: Pizza {
        topping: Some(PizzaTopping::Cheese) | Some(PizzaTopping::Pineapple) | None
    }
} => {} };
