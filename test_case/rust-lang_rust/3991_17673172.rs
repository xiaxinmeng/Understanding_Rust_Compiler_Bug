 rust
struct HasNested {
    nest: ~[~[int]],
}

impl HasNested {

    fn method_push_local(&mut self) {
        self.nest[0].push(0);
    }

}

fn main() {}
