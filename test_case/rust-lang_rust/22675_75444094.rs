
struct Option_;
impl<T> TTrans<T> for Option_ { type O = Option<T>; }
impl<T> Other<Option_> for Option<T> {
  type E = T;
  fn ap2<R, F : Fn(T)->R>(self, f : F) -> Option<R> {
    match self {
      None => None,
      Some(x) => Some(f(x))
    }
  }
}
fn main() {
  let x : Option<isize> = Some(3);
  let v : Option<isize> = x.ap(|x| { x + 3 });  // must use the generic ap -> ap2 adapter
  match v {
    None => {},
    Some(y) => { println!("{}", y); }
  }
}
