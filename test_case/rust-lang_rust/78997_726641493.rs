rust
struct Horse;
struct Poop;
trait Animal { fn poop(&mut self) -> Poop { Poop } }
impl Animal for Horse { }

fn main() {
  let mut horse = Horse;
  horse.poop();
}
