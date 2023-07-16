
// lifetimes.rs

extern crate debug;

pub trait FromSlice<'a> {
    fn new(struct_reader : &'a [u8]) -> Self;
}

pub struct MessageReader {
    buf : Vec<u8>,
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
        let message = MessageReader {buf : vec!(1,2,3,4,5)};
        let foo : FooReader = message.get_root();
        foo
    }; // this shouldn't typecheck, because `foo` cannot outlive `message`

    // prints "bar = FooReader<'_>{reader: &[96u8, 16u8, 129u8, 7u8, 1u8]}"
    println!("bar = {:?}", bar);
}
