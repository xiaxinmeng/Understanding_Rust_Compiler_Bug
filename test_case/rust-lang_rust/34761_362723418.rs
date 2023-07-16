rust
struct Bar<'a>(*const String, PhantomData<&'a()>);
impl<'a> Bar<'a> {
    fn new(x: &'a String) -> Bar<'a> { Bar(x, PhantomData) }
}
impl<'a> Deref for Bar<'a> {
    Target = String;
    fn deref(&self) -> &String { unsafe { &*self.0 } }
}
unsafe impl<#[may_dangle] 'a> Drop for Bar<'a> {
    fn drop(&mut self) { unsafe { println!("{}", &*self.0) } }
}
