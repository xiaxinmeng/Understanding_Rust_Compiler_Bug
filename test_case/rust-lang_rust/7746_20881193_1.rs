
pub struct FooIterator {
    priv state: uint,
    priv m: uint
}

impl Iterator<uint> for FooIterator {
    fn next(&mut self) -> Option<uint> {
        let (new_state, ret) = match self.state {
            0 => (1, 1) 
            1 => (2, 2) 
            2 => (0, self.m)
            _ => fail!()
        };
        self.state = new_state;
        ret
    }
}

fn foo(m: uint) -> FooIterator {
    FooIterator{ state: 0, m: m }
}
