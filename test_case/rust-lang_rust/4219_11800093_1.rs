 rust
// integer-templates.rs
{

macro_rules! int_template (

  ($typ:ty, $bits:expr) => {

    pub mod inst { // FIXME(#3086): Macro expansion to multiple items

      use char;
      use cmp;
      use cmp::{Eq, Ord};
      use from_str::FromStr;
      use iter;
      use num;
      use num::Num::from_int;
      use str;
      use uint;
      use vec;

      pub type T = $typ;
      pub const bits  : uint = $bits;
      pub const bytes : uint = (bits / 8);

      // rest of int-template.rs code
    }
  }
)

macro_rules! uint_template (
  // etc
)

}
