 rust
/home/ktt3ja/dead-code-test/test4.rs:1:0: 1:11 warning: code is never used: `Foo`, #[warn(dead_code)] on by default
/home/ktt3ja/dead-code-test/test4.rs:1 struct Foo;
                                       ^~~~~~~~~~~
/home/ktt3ja/dead-code-test/test4.rs:2:11: 2:34 warning: code is never used: `foo`, #[warn(dead_code)] on by default
/home/ktt3ja/dead-code-test/test4.rs:2 impl Foo { fn foo(&self) { bar() } }
                                                  ^~~~~~~~~~~~~~~~~~~~~~~
/home/ktt3ja/dead-code-test/test4.rs:3:0: 8:1 warning: code is never used: `bar`, #[warn(dead_code)] on by default
/home/ktt3ja/dead-code-test/test4.rs:3 fn bar() {
/home/ktt3ja/dead-code-test/test4.rs:4    fn baz() {}
/home/ktt3ja/dead-code-test/test4.rs:5 
/home/ktt3ja/dead-code-test/test4.rs:6    Foo.foo();
/home/ktt3ja/dead-code-test/test4.rs:7    baz();
/home/ktt3ja/dead-code-test/test4.rs:8 }
/home/ktt3ja/dead-code-test/test4.rs:4:3: 4:14 warning: code is never used: `baz`, #[warn(dead_code)] on by default
/home/ktt3ja/dead-code-test/test4.rs:4    fn baz() {}
                                          ^~~~~~~~~~~
