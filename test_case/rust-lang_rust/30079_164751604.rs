
struct PrivOuter;
trait PrivOuterTrait {
    type Output;
    fn reveal(&self) -> Self::Output;
}

mod m {
    struct PrivInner;
    impl PrivInner {
        pub fn secret(&self) { println!("Hello!"); }
    }

    impl super::PrivOuterTrait for super::PrivOuter {
        type Output = PrivInner;
        fn reveal(&self) -> PrivInner { PrivInner }
    }

}

fn main() {
    PrivOuter.reveal().secret();
}
