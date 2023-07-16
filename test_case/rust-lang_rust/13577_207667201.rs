
src/foo.rs:2:5: 2:8 error: cannot declare a new module at this location
src/foo.rs:2 mod bar;
                 ^~~
src/foo.rs:2:5: 2:8 note: maybe move this module `foo` to its own directory via `foo/mod.rs`
src/foo.rs:2 mod bar;
                 ^~~
src/foo.rs:2:5: 2:8 note: ... or maybe `use` the module `bar` instead of possibly redeclaring it
src/foo.rs:2 mod bar;
                 ^~~
