rust
struct Chicken<'a, T> where T: Trait { f: &'a Frog<'a, T::Item> }

struct Frog<'a, T> where T: Trait  { f: &'a Chicken<'a, T> }

trait Trait {
    type Item;
}
