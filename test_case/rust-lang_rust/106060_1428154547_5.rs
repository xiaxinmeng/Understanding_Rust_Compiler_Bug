rust\"] #[doc = r\" # extern cra\
te rental;\"]\n    #[doc = r\" # use rental::common::RentMutex;\"] #[doc = r\" # fn main() {\"]\n    #[doc = r\" use std::sync;\"] #[doc = r\"\"]\n    #[doc =\n    \
r\" let mut r = RentMutex::new(Box::new(sync::Mutex::new(5)), |c| c.lock().unwrap());\"]\n    #[doc = r\" *r = 12;\"]\n    #[doc = r\" assert_eq!(12, RentMutex::ren\
t(&r, |c| **c));\"]\n    #[doc = r\" # }\"] #[doc = r\" 