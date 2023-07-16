 rust
  pub trait Sum {
      fn sum<I: Iterator<Item=Self>>(iter: I) -> Self;
  }
  
  pub trait Product {
      fn product<I: Iterator<Item=Self>>(iter: I) -> Self;
  }
  