rust
trait Bounds: Default + std::fmt::Debug {}
impl<T> Bounds for T where T: Default + std::fmt::Debug {}

trait MyTrait {
    //type Associated: Bounds = ();

    fn call_with_associated<C: WithAssociated>(callback: C) -> C::Value {
        type DefaultAssociated = ();
        callback.run::<DefaultAssociated>()
    }
}

trait WithAssociated {
    type Value;
    fn run<A: Bounds>(self) -> Self::Value;
}

struct T1;
impl MyTrait for T1 {
    // type Associated = ();
}

struct T2;
impl MyTrait for T2 {
    // type Associated = String;

    fn call_with_associated<C: WithAssociated>(callback: C) -> C::Value {
        callback.run::<String>()
    }
}

fn main() {
    struct Callback;
    impl WithAssociated for Callback {
        type Value = ();
        fn run<A: Bounds>(self) -> Self::Value {
            // code uses the "associated type" A
            println!("Associated = {:?}", A::default());
        }
    }
    T1::call_with_associated(Callback);
    T2::call_with_associated(Callback);
}
