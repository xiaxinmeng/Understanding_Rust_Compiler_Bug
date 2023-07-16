
trait Foo {
    fn foo<'a>(x: &mut Vec<&'a u8>, y: &'a u8);
}
impl Foo for () {

    fn foo(x: &mut Vec<&u8>, y: &u8) {
       x.push(y);
   }
}
