
   pub fn baz() {
       mod z {
           pub fn f () -> int { 19 }
       }
       fn g() -> int { use z; z::f() }
       g();
   }
