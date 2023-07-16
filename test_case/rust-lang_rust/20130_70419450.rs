
// If foo.rs only generates one file, it will be called precisely `foo`
$ rustc foo.rs -o foo
// Use `foo` as a filestem for all outputs, ignore the precise filename "foo"
$ rustc --emit=[lots] foo.rs -o foo
