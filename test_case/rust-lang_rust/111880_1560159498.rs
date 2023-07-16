
fn is_pointer_like<T: PointerLike>(_: T) {}

fn foo<T: 'a>() {
  let x: &'static () = &();
  is_pointer_like(x);
}
