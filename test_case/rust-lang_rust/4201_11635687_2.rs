
$ rustc example.rs 
example.rs:9:11: 9:33 error: mismatched types: expected `()` but found `~[()]` (expected () but found vector)
example.rs:9         do map([0, 1, 2, 3, 4]) |i| {
                        ^~~~~~~~~~~~~~~~~~~~~~
