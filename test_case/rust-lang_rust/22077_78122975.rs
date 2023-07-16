 rust


struct Foo<'a> {
    inner: &'a mut [u8]
}

impl<'a, 'b> Iterator for Foo<'a> {
    type Item = &'b mut u8;

    fn next(&mut self) -> Option<&mut u8> {
        Some(&mut self.inner[0])
    }
}

fn main() {
    let mut fun = [1,2,3,4,5];
    let mut foo = Foo {
        inner: &mut fun
    };
    {
        let a = foo.next().unwrap();
        let b = foo.next().unwrap();
        *a = 1;
        *b = 2;
        println!("{}", a);
    }
}
