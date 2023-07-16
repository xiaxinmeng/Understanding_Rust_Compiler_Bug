rust
cmp("test")(&&"test".to_string())
cmp("test")(&"test")//This works, because str and Path are both not Sized, right?
