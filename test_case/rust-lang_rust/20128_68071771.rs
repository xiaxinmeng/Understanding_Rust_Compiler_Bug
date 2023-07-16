
trait MyTrait {}

struct MyStruct;

impl MyStruct {
    fn new() -> Result<MyStruct, String> {
        return Ok(MyStruct);
    }
}

impl MyTrait for MyStruct {}

fn get() -> Result<(), String> {

    box match MyStruct::new() {
        Ok(v) => v,
        Err(e) => return Err(e),
    } as Box<MyTrait>;

    return Ok(());
}

fn main() {
}
