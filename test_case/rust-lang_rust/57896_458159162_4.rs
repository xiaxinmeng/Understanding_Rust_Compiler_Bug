rust
let _: (&str, &str) = foo("foo"); // Foo<&str, &str>
let _: (&str, i32) = bar("", 42); // Foo<&str, i32>
let _: (i32, &str) = mep(42, ""); // Foo<&str, i32>
let _: (i32, &str) = bar(42, ""); // Foo<i32, &str>
let _: (&str, i32) = mep("", 42); // Foo<i32, &str>
