
mod foo {
  struct Bar {
    ....
  }
}

pub use foo:Bar;

derive!(Bar, (Clone))
