rust
trait NeedsBody { fn g(); }
struct S;
fn f() {
   impl NeedsBody for S {
       fn g() {
           fn doubly_nested(a: UnknownType) {}
       }
   }
}
