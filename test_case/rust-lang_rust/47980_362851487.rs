rust
pub trait U16Constant {
  const X: u8;
}

pub fn barbaz<T: U16Constant>(x: u16) {
   coresimd::barbaz(x, T::X)
}
