rust\"] #[doc = r\" # extern crate rental;\"]\n    #[doc = r\" # use rental::common::R\
entRwLock;\"] #[doc = r\" # fn main() {\"]\n    #[doc = r\" use std::sync;\"] #[doc = r\"\"]\n    #[doc =\n    r\" let r = RentRwLock::new(Box::new(sync::RwLock::ne\
w(5)), |c| c.read().unwrap());\"]\n    #[doc = r\" assert_eq!(*r, RentRwLock::rent(&r, |c| **c));\"]\n    #[doc = r\" # }\"] #[doc = r\" 