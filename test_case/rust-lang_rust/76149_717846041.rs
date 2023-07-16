rust
struct HoldsReference<'a> {
    reference: &'a u32,
}

// must have manual drop impl
impl<'a> std::ops::Drop for HoldsReference<'a> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

struct Error {/* ... */}

fn create<'a>(_reference: &'a u32) -> Result<HoldsReference<'a>, Error> {
    unimplemented!()
}

struct Owner {
    owned: u32,
}

impl Owner {
    // must be `mut self`
    fn on_error(&mut self, _error: Error) {
        unimplemented!()
    }

    // All of the following functions seem equivalent but only the last compiles.

    fn fails_to_compile_0(&mut self) {
        if let Err(err) = create(&self.owned) {
            self.on_error(err)
        }
    }

    fn fails_to_compile_1(&mut self) {
        match create(&self.owned) {
            Ok(_) => (),
            Err(err) => self.on_error(err),
        }
    }

    fn fails_to_compile_2(&mut self) {
        match create(&self.owned) {
            Ok(_) => return,
            Err(err) => self.on_error(err),
        }
    }

    fn compiles(&mut self) {
        let err = match create(&self.owned) {
            Ok(_) => return,
            Err(err) => err,
        };
        self.on_error(err);
    }
}

fn main() {}
