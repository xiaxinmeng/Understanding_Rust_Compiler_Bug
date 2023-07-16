 rust
pub struct Node<'a,T:'a+Sized+PartialOrd>
{
    pub data:Rc<Box<[Option<&'a T>]>>,
}
