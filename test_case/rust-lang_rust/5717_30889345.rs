
foo.rs:7:8: 7:9 error: mismatched types: expected `@mut <VI1>` but found `@<VI0>` (values differ in mutability)
foo.rs:7     d = a;      // error doesn't appear if this isn't mentioned
                 ^
error: aborting due to previous error
