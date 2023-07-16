rust\"] #[doc\
 = r\" # extern crate rental;\"]\n    #[doc = r\" # use rental::common::RentRwLockMut;\"]\n    #[doc = r\" # fn main() {\"] #[doc = r\" use std::sync;\"] #[doc = r\\
"\"]\n    #[doc =\n    r\" let mut r = RentRwLockMut::new(Box::new(sync::RwLock::new(5)), |c| c.write().unwrap());\"]\n    #[doc = r\" *r = 12;\"]\n    #[doc = r\" \
assert_eq!(12, RentRwLockMut::rent(&r, |c| **c));\"]\n    #[doc = r\" # }\"] #[doc = r\" 