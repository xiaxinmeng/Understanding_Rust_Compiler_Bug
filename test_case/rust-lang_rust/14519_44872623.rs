 rust
use std::f32;
use std::f64;

fn main()
{
  let o = from_str::<f64>("-1.7976931348623158e+308").unwrap();
  if o.is_negative() && o.is_infinite() {
    println!("OK {}", o);
  }
  else {
    println!("NOK {}", o);
  }
}
