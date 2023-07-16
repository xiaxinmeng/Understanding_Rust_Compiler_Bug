 cmd
$ ./rustc test.rs
test.rs:2:5: 2:10 error: mismatched types:
           expected `core::option::Option<int>`,
              found `core::result::Result<_, _>`
          (expected `enum core::option::Option`,
              found `enum core::result::Result`)
test.rs:2     Ok(7) // Should be Some(7)
              ^~~~~
error: aborting due to previous error
