rust
#[cold]
fn panicking_stuff(__capture0: Capture<???>, __capture1: Capture<???>, __capture2: Capture<???>) {
 panic!( ... )
}

if ! assert_expression {
  panicking_stuff(__capture0, __capture1, __capture2);
}
