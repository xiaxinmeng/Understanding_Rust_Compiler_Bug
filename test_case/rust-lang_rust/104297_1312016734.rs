rust
struct Foo([u8]);

trait Bar {}

impl Bar for Foo {}

const _: () = {
    trait Blanket {}
    impl<T: Bar + ?Sized> Blanket for T {}
    impl Blanket for Foo {}
    //~^ ERROR conflicting implementations of trait `Blanket` for type `Foo`
};
