 console
$ echo 'mod foo;' > test.rs
$ rustc ./test.rs
test.rs:1:5: 1:8 error: file not found for module `foo`
test.rs:1 mod foo;
              ^~~
test.rs:1:5: 1:8 help: name the file either foo.rs or foo/mod.rs inside the directory ""
