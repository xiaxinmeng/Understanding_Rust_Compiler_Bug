rust\"] #[doc = r\" \
# extern crate rental;\"]\n    #[doc = r\" # use rental::common::RentRefCell;\"] #[doc = r\" # fn main() {\"]\n    #[doc = r\" use std::cell;\"] #[doc = r\"\"]\n   \
 #[doc =\n    r\" let r = RentRefCell::new(Box::new(cell::RefCell::new(5)), |c| c.borrow());\"]\n    #[doc = r\" assert_eq!(*r, RentRefCell::rent(&r, |c| **c));\"]\\
n    #[doc = r\" # }\"] #[doc = r\" 