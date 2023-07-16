
trait Add {
    fn to_int(&self) -> int;
    fn add_dynamic(&self, other: &Add) -> int;
}

impl int: Add {
    fn to_int(&self) -> int { *self }
    fn add_dynamic(&self, other: &Add) -> int {
        self.to_int() + other.to_int()
    }
}

fn main() { }
