
Running /usr/local/bin/rustc:
/Users/nmatsakis/tmp/bug3501.rs:12:20: 12:21 error: illegal borrow: borrowed value does not live long enough
/Users/nmatsakis/tmp/bug3501.rs:12     push(&mut a.y, &a.x);
                                                       ^
/Users/nmatsakis/tmp/bug3501.rs:10:12: 14:1 note: borrowed pointer must be valid for the anonymous lifetime #1 defined on the block at 10:12...
/Users/nmatsakis/tmp/bug3501.rs:10 fn A() -> A {
/Users/nmatsakis/tmp/bug3501.rs:11     let a = A { x: ~1, y: ~[] };
/Users/nmatsakis/tmp/bug3501.rs:12     push(&mut a.y, &a.x);
/Users/nmatsakis/tmp/bug3501.rs:13     a
/Users/nmatsakis/tmp/bug3501.rs:14 }
/Users/nmatsakis/tmp/bug3501.rs:10:12: 14:1 note: ...but borrowed value is only valid for the block at 10:12
/Users/nmatsakis/tmp/bug3501.rs:10 fn A() -> A {
/Users/nmatsakis/tmp/bug3501.rs:11     let a = A { x: ~1, y: ~[] };
/Users/nmatsakis/tmp/bug3501.rs:12     push(&mut a.y, &a.x);
/Users/nmatsakis/tmp/bug3501.rs:13     a
/Users/nmatsakis/tmp/bug3501.rs:14 }
error: aborting due to previous error
