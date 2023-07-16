
struct TickyTacky {
    data: uint,
    neighbour: @mut Option<TickyTacky>,
}

impl Drop for TickyTacky {
    fn finalize(&self) {
        if (self.neighbour.is_some()) {
                let other_box = option::swap_unwrap(self.neighbour);
                fail_unless!(other_box.data == 42 || other_box.data == 31337);
        }
    }
}

fn main() {
    let box1 = @mut Some(TickyTacky { data: 42, neighbour: @mut None });
    let box2 = @mut Some(TickyTacky { data: 31337, neighbour: box1 });

    *box1 = Some(TickyTacky { data: 42, neighbour: box2 });
}
