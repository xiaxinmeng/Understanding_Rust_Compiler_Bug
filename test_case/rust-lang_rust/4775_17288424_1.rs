
./foo.rs:25:24: 25:25 error: illegal borrow: borrowed value does not live long enough
./foo.rs:25     let r = squarei(3, &f as &OpInt);
                                    ^
note: borrowed pointer must be valid for the static lifetime...
./foo.rs:22:10: 27:1 note: ...but borrowed value is only valid for the block at 22:10
./foo.rs:22 fn main() {
./foo.rs:23     echo("Entered main");
./foo.rs:24     let f: &fn(int, int) -> int = |x, y| muli(x, y);
./foo.rs:25     let r = squarei(3, &f as &OpInt);
./foo.rs:26     echo(r);
./foo.rs:27 }
error: aborting due to previous error
