
2. src/lib.rs:x:y | assert_send(foo());

    defines `Foo` as `Send` because of the definition of `assert_send`
