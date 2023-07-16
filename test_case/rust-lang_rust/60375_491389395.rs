rust
use std::any::Any;
use std::any::TypeId;

trait Resource {
}

trait ResourceT {
    fn type_id(&self) -> TypeId;
}

struct TestResourceT {
    value: u32
}

impl ResourceT for TestResourceT {
    fn type_id(&self) -> TypeId { TypeId::of::<TestResource>() }
}

struct TestResource {}

fn main() {
    let test = TestResourceT{ value: 10 };

    println!("{:?}", TypeId::of::<TestResource>());
    println!("{:?}", test.type_id());
}
