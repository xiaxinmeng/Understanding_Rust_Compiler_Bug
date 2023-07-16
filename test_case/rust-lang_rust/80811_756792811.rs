rust

trait MyTrait {}

struct Foo{}

impl MyTrait for Foo {}


fn bar()-> Result<Box<dyn MyTrait>, ()> {
    Ok(Box::new(Foo {}))
}
