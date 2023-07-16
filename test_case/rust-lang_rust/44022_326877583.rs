rust
#![crate_type="lib"] 
#![feature(specialization)] 

trait X {} 
trait Y: X {} 
trait Z { type Assoc: Y; }

struct A<T>(T);

impl<T: X> Z for A<T> { type Assoc = T; }
impl<T: X> From<<A<T> as Z>::Assoc> for T {}
