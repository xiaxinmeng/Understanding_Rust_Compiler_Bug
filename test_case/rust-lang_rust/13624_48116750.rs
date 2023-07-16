 rust
#![feature(struct_variant)]

mod a {
  pub enum Enum {  
    EnumStructVariant { x: u8, y: u8, z: u8 }
  }

  pub fn get_enum_struct_variant() {
    EnumStructVariant { x: 1, y: 2, z: 3 }
//~^ ERROR mismatched types: expected `()` but found `a::Enum` (expected () but found enum a::Enum)
  }
}

mod b {
  mod test {
    use a;

    fn test_enum_struct_variant() {
      let enum_struct_variant = ::a::get_enum_struct_variant();
      match enum_struct_variant {
        a::EnumStructVariant { x, y, z } => {
        //~^ ERROR error: mismatched types: expected `()` but found a structure pattern
        }
      }
    }
  }
}

fn main() {}
