 rust
use missing;

impl Test {
    pub fn test(&self, missing: &A) -> Option<Vec<missing::B>> {
        match *missing {
            A::Missing{keycode, ..} => {
                unimplemented!()
            },
            _ => {}
        }
        None
    }
}
