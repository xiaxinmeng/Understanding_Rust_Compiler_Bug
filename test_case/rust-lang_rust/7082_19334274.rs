 rust
struct Foo {
  name: ~str,
  other: Option<@Foo>
}

fn circle(s:~str) -> @Foo {
  unsafe {
    let f = @Foo{ name:s, other:None};
    let x = Some(f);
    std::ptr::copy_nonoverlapping_memory::<Option<@Foo>>(std::unstable::intrinsics::transmute(&f.other), &x, 1);
    f
  }
}

fn main() {
  let f = ~circle(~"Hi");
  println(f.name);
  println(f.other.get().name);
  println(f.other.get().other.get().name);
}
