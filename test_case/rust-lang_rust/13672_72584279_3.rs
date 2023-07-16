
/t/triage $ rustc a.rs
/t/triage $ rustc b.rs -L.
c.rs:2:5: 2:6 error: unresolved import `a::Foo`. Did you mean `self::a`?
c.rs:2 use a::Foo;
           ^
error: aborting due to previous error
