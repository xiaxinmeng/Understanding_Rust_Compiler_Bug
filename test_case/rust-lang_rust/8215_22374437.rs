 rust
pub mod inner_api_a {
   use super::inner_common_impl;
   pub fn api1() {
       inner_common_impl::common1()
   }
   pub fn api2() {
       inner_common_impl::common2()
   }
}

pub mod inner_api_a {
   use super::inner_common_impl;
   pub fn api1() {
       inner_common_impl::common3()
   }
   pub fn api2() {
       inner_common_impl::common4()
   }
}

mod inner_common_impl {
    fn common1() { }
    fn common2() { }
    fn common3() { }
    fn common4() { }
}
