rust
union Foo {
  x: ManuallyDrop<String>,
  y: ManuallyDrop<Box<i32>>
}
