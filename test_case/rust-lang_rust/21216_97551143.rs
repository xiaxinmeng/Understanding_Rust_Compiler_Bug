 rust
fn attach_fn(opt_fn: &mut Box<FnMut()>) {
  let mut option = None;
  *opt_fn = Box::new(|| { option = Some(7) });
}

fn main() {
  let mut optional_fn = Box::new(|| {});
  attach_fn(&mut optional_fn);
}
