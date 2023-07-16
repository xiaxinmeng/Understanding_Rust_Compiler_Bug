
#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Hash,Debug)]
pub struct PrimeUsize(usize);

impl PrimeUsize {
    fn new(usize)->Self { /* Primality check here */ }
}

impl Deref for PrimeUsize {
    type Target = usize;
    fn deref(&self)->&usize { &self.0 }
}
