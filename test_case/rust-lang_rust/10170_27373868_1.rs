 rust
struct Master;

struct Child<'self>
{
  master: &'self mut Master
}

fn main(){
  let mut m=Master;
  let _=Child{master: &mut m};
  let _=Child{master: &mut m};
}
