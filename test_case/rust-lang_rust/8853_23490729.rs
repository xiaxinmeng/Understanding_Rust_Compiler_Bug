 rust
fn main() {
    let x = ~"foo";
    let y = ~"bar";
    println(x.add(y));
}

/* z.rs:4:15: 4:16 error: mismatched types: expected `&&str` but found `~str` (expected &-ptr but found ~str)
z.rs:4  println(x.add(y));
                      ^
*/
