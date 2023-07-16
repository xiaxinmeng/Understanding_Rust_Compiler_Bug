rust
trait Animal {}

struct Horse;
impl Animal for Horse {}

// Using `impl Trait` here to avoid unnecessary heap allocation if `k < 10`
fn make_animal(set: impl FnOnce(u32)) -> impl Animal {
    set(3);
    Horse // alternatively, return value of complex/unnamed type that impls Animal
}

fn expand_zoo(zoo: &mut Vec<Box<dyn Animal>>) {
    let mut k = 0;
    let x = make_animal(|r| k = r);
    if k >= 10 {
        zoo.push(Box::new(x));
    }
}
