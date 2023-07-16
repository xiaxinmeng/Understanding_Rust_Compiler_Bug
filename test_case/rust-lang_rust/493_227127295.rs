
pub fn baz() {
   mod z {
       pub fn f () -> i32 { 19 }
   }
   fn g() -> i32 { z::f() }
   g();
}
