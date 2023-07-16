rust
#![feature(slice_patterns)]

fn main() {
   let x: &[u8] = &[0];
   match x { // this code compiles
       &[] => {},
       &[1..=255] => {},
       b"\x00" => {}, //~ unreachable pattern (no it's not)
       &[_, _, ..] => {}
   }
}
