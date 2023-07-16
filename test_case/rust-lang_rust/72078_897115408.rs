rust
trait Foo{
fn dummy()->Self{unimplemented!("don't judge me, I just wanted to hack without needing to implement everything to compile")}
fn bar()->Self{
dummy()
}
}
#[derive(Debug)]
struct Bar{
//some data
}
impl Foo for Bar{
fn bar()->Self{
 Bar{
//correct construction
}
}

}
fn main(){
println!("{:?}",Bar::bar()); //this will crash with the unimplemented! message because it still calls dummy
}
