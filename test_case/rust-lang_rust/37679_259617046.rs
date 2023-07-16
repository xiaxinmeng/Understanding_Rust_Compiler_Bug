
macro_rules! documented {
  ($(#[$attr:meta])*, $func:ident) => {
    $(#[$attr])*
    fn $func () {
    }
  }
}

documented!(#[doc = "Function foo"], foo);
documented!(
/// Function bar
, bar);
