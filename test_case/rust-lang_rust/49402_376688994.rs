rust
impl<A, B> Iterator for Chain<A, B> 
where
    A: Iterator,
    B: Iterator<Item = <A as Iterator>::Item>, 
