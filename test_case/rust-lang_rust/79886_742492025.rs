rust
use core::cell::UnsafeCell;

struct Scoped<'a>(UnsafeCell<Option<&'a Scoped<'a>>>);

fn scoped<'a: 'b, 'b>(s: &'b Scoped<'a>) -> Scoped<'b> {
    Scoped(UnsafeCell::new(Some(s)))
}
