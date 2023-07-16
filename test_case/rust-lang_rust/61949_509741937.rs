rust
#![feature(async_await)]

pub struct Foo<'a> {
    pub bar: &'a i32,
}

impl<'a> Foo<'a> {
    pub async fn new(_bar: &'a i32) -> Self {
        Foo {
            bar: &22
        }
    }
}

async fn foo() {
  let x = {
    let bar = 22;
    Foo::new(&bar).await
  };
  drop(x);
}

fn main() { }
