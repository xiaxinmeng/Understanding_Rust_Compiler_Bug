rust
#![feature(specialization)]

use std::iter::Iterator;

trait IntoPyDictPointer { }

struct Foo { }

impl Iterator for Foo {
  type Item = ();
  fn next(&mut self) -> Option<()> {
      None
  }
}

impl IntoPyDictPointer for Foo { }

impl<I> IntoPyDictPointer for I
where
    I: Iterator,
{
}

impl IntoPyDictPointer for ()
{
}

fn main() { }
