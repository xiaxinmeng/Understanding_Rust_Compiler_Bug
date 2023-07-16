rust
#![feature(test)]

extern crate test;

use std::{
    cell::Cell,
    fmt::{self, Write},
};

struct NoOpWrite;

impl Write for NoOpWrite {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }
}

#[bench]
fn debugmap_entry(b: &mut test::Bencher) {
    struct Map<'a>(Cell<Option<&'a mut test::Bencher>>);

    impl<'a> fmt::Debug for Map<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut map = f.debug_map();

            let b = self.0.take().unwrap();
            b.iter(|| {
                map.entry(&"a", &42);
            });

            map.finish()
        }
    }

    let _ = write!(NoOpWrite, "{:?}", Map(Cell::new(Some(b))));
}
