
// lifetimes.rs

pub trait FromSlice<'a> {
    fn new(struct_reader : &'a [u8]) -> Self;
}

pub struct MessageReader {
    buf : ~[u8],
}

impl MessageReader {

    pub fn get_root<'a, T : FromSlice<'a>>(&'a self) -> T {
        FromSlice::<'a>::new(self.buf.as_slice())
    }

}

pub struct FooReader<'a> {
    reader : &'a [u8]
}

impl <'a> FromSlice<'a> for FooReader<'a> {
    fn new(reader : &'a [u8]) -> FooReader<'a> {
        FooReader { reader : reader}
    }
}

pub fn main () {
    let bar = {
        let message = MessageReader {buf : ~[1,2,3,4,5]};
        let foo : FooReader = message.get_root();
        foo
    }; // this shouldn't typecheck, because `foo` cannot outlive `message`

    let _x : ~[u8] = ~[9,9,9,9,9,9,9,9,9,9,9,9];

    println!("bar = {:?}", bar);
}
