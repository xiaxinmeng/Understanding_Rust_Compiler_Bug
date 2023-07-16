
warning: `Whatever` has fields that are never read
  --> src\cmake\preset.rs:23:3
   |
23 |   field1: (),
   |   ^^^^^^^^^^
24 |   field2: (),
   |   ^^^^^^^^^^
25 |   field3: (),
   |   ^^^^^^^^^^
26 |   field4: (),
   |   ^^^^^^^^^^
   |
note: `Whatever` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  --> src\cmake\preset.rs:20:10
   |
20 | #[derive(Debug)]
   |          ^^^^^
   = note: this warning originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
