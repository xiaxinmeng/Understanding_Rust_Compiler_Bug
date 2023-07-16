 rust
struct Foo {
  a: uint,
}

fn main(){
  let bar = Foo {a: 29};

  let baz = {
    let Foo {a: _, a: _} = bar;
  };
}
