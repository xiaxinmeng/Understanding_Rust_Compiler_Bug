rust
impl<'a, 'b> Foo for &'b mut Quix<'a>
where
   'a: 'b, 
{
    type Ty = impl Printy;

    fn bar(self) -> Self::Ty {
        self
    }
}
