
mod upstream {
    pub trait Foo<A> {
        fn execute(self) -> A;
    }
}

mod impl_one {
    use super::upstream::*;
    struct OneS;
    impl Foo<OneS> for String {
        fn execute(self) -> OneS {
            OneS {}
        }
    }
}

use upstream::*;
fn main(){
    let a = "foo".to_string().execute();
}
