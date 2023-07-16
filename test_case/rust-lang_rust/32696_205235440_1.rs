
pub trait Trait {    
    fn method(&self);
}

macro_rules! count{
    () =>{{
        struct COUNT {}
        static C: COUNT = COUNT {};
        let _=&C;
    }}
}

impl<T> Trait for T {
    fn method(&self) {
        let _=|| count!();
    }
}
