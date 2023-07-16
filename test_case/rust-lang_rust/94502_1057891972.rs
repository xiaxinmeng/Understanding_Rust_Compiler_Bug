rust
struct Repro;
impl Repro {
    fn get(&self) -> &i32 {
        &3
    }

    fn insert(&mut self, _: i32) {}
}

fn main() {
    let x = &0;
    let mut conflict = Repro;
    let prev = conflict.get();
    conflict.insert(*prev + *x);
}
