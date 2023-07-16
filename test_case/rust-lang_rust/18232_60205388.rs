 rust
#![allow(dead_code)]

struct Cursor<'a>;

trait CursorNavigator {
    fn init_cursor<'a, 'b:'a>(&'a self, cursor: &mut Cursor<'b>) -> bool;
}

struct SimpleNavigator;

impl CursorNavigator for SimpleNavigator {
    fn init_cursor<'a, 'b: 'a>(&'a self, _cursor: &mut Cursor<'b>) -> bool {
        false
    }
}

fn main() {
    let mut c = Cursor;
    let n = SimpleNavigator;
    n.init_cursor(&mut c); // <-------------------- fails on resolving this method call
}
