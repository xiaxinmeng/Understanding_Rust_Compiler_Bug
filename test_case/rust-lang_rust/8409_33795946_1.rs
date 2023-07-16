 rust
macro_rules! unsafe_fields {
    (<$('$lifetime: ident, )* 
      $($param: ident),*>
     $ty: ty: $( $field: ident: $field_ty: ty [$setter: ident] ),*) => {
        impl<$($param)*> $ty {
            $(
                pub fn $field<'a>(&'a self) -> &'a $field_ty { &self.$field }
                pub unsafe fn $setter(&mut self, x: $field_ty) {
                    self.$field = x
                }
            )*
        }
    }
}

unsafe_fields! {
    <'a, S, T> Attribute<'a, S, T>:
    context: &'a Context [set_context],
    contents: gl::GLuint [set_contents]
}
