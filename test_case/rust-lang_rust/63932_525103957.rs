
struct Inventory;

impl Inventory {
    pub fn index(&self) -> &bool {
        &true
    }
}

fn main() {
    let game_state = Inventory;

    loop {
        match game_state {
            ref inventory if *inventory.index() => {}
            ref inventory if *inventory.index() => {}
            ref mut inventory => {}
        }
    }
}

