 rust
   // error-pattern:thread '<main>' panicked at 'shift operation overflowed'
   // compile-flags: -C debug-assertions
   
   // (Work around constant-evaluation)
   const fn id<T>(x: T) -> T { x }
   
   fn main() {
       let _x = -1_i32 >> id(32);
   }
   