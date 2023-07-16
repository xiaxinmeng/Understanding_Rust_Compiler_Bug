rust
pub struct Foo(Cell<bool>);

impl Deref for Foo{
     type Target = bool;
     pub fn deref(&self) -> &bool{
          const RESOLVED: &[bool;2] = &[false,true];
          let val = self.0.get();
          self.0.set(!val);
          &RESOLVED[val as usize]
     }
}

let foo = Foo(Cell::new(false));

match &foo{
     true => println!("{}",foo.get()),
     false => println!("{}",foo.get()),
}
