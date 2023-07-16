
fn foo<T: Fn()>(t: T) { t() }

fn bar() {
  let mut x = 3;
  foo(|| x += 1);
}
