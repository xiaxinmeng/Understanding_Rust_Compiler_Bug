rust
macro_rules! def_struct {
  ($name:ident $($body:tt)*) => {
    pub struct $name $($body)*
  };
}

def_struct!(X {}); // compiles
def_struct!(Y ());

fn main() {}
