rust
#[derive(Debug)]
struct Type {
    field1 : u64,
    field2 : u32,
    field3 : Vec<u32>,
}

impl Type   {
    #[no_mangle]
    //#[inline(never)]
    fn mut_update(&mut self, field2 : u32) -> &mut Self {
        self.field2 = field2;
        self
    }

    #[no_mangle]
    //#[inline(never)]
    fn func_update(self, field2 : u32) -> Self  {
        Type { field2 : field2, .. self }
    }
}

fn main()   {
    let mut var = Type { field1 : 42, field2 : 0, field3 : vec![12] };

    let _ = var.mut_update(79);
    println!("{:?}", var);
    let _other = var.func_update(112);
    println!("{:?}", _other);
}
