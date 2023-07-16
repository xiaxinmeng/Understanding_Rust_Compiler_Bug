rust
/// Swallow dropck requirements of T while still dropping it, because
/// `ManuallyDrop` doesn't own `T` as far as dropck is concerned.
struct Wrapper<T>(ManuallyDrop<T>);

unsafe impl<#[may_dangle] T> Drop for Wrapper<T> {
  fn drop(&mut self) {
    unsafe {
      ManuallyDrop::drop(&mut self.0);
    }
  }
}

struct PrintOnDrop<'a>(&'a str);
impl<'a> Drop for PrintOnDrop<'a> {
  fn drop(&mut self) {
    println!("{}", self.0);
  }
}

fn main() {
  let s = String::from("hello");
  let pod_wrapper = Wrapper(ManuallyDrop::new(PrintOnDrop(&*s)));
  drop(s);
}
