rust
struct S;

impl std::convert::TryInto<i32> for S {
    type Error = ();
    fn try_into(self) -> Result<i32, Self::Error> {
        Err(())
    }
}

mod out_of_scope {
    pub trait TryInto {
        fn try_into(self) -> Result<i32, ()>;
    }
    
    impl TryInto for super::S {
        fn try_into(self) -> Result<i32, ()> {
            Err(())
        }
    }
}

fn err(a: S) -> i32 {
    a.try_into().unwrap()
}
