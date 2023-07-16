rust
union Foo { bar: i8, zst: (), pizza: Pizza, oneval: OneVal, twoval: TwoVal, khar: char }

#[derive(Copy, Clone)]
struct Pizza { topping: Option<PizzaTopping> }

#[derive(Copy, Clone)]
enum PizzaTopping { Cheese, Pineapple }

#[derive(Copy, Clone)]
#[repr(u8)]
enum OneVal { One = 1 }

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum TwoVal {
    One = 1,
    Two = 2,
}

let mut foo = Foo { bar: 5 };
