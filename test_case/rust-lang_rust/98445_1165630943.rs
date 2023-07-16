rust
#[inline]
fn clone(&self)->Self{
   Self(::core::clone::Clone::clone(&self.0), ..., ::core::clone::Clone::clone(&self.613),)
}

#[inline]
fn clone(&self, other: &Self){
   ::core::clone::Clone::clone_from(&mut self.0, &other.0);
   ...
   ::core::clone::Clone::clone_from(&mut self.613, &other.613);
}
