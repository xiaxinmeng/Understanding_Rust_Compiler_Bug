
pub struct StructReader<'a> {
    buf : *u8,
    len : uint,
    marker : std::kinds::marker::ContravariantLifetime<'a>,
}

pub trait FromStructReader<'a> {
    fn new(struct_reader : StructReader<'a>) -> Self;
}

pub struct MessageReader {
    buf : ~[u8],
}

impl MessageReader {
    pub fn get_root<'a, T : FromStructReader<'a>>(&'a self) -> T {
        FromStructReader::<'a>::new(
            StructReader {buf : self.buf.as_ptr(),
                          len : self.buf.len(),
                          marker : std::kinds::marker::ContravariantLifetime::<'a>,
            })
    }
}

pub struct FooReader<'a> {
    reader : StructReader<'a>
}

impl <'a> FromStructReader<'a> for FooReader<'a> {
    fn new(struct_reader : StructReader<'a>) -> FooReader<'a> {
        FooReader { reader : struct_reader}
    }
}

pub fn main () {
    let message = MessageReader {buf : ~[1,2,3,4,5]};
    let foo : FooReader = message.get_root();
}
