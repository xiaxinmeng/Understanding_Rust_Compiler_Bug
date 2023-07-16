 rust
fn invoke_fn<F>(f: &mut Box<&mut F>) where F: FnMut() {
  (*f)()
}

fn main() {
  let mut option = None;
  let mut fn_mut = || {
    option = Some(7)
  };
  let mut boxed_fn = Box::new(&mut fn_mut);

  invoke_fn(&mut boxed_fn);
  invoke_fn(&mut boxed_fn);
}
