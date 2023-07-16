rs
struct GenericAssocMethod<T>(T);
impl<T> GenericAssocMethod<T> {
  fn default_hello() {}
}
let x = GenericAssocMethod(33);
x.default_hello();
