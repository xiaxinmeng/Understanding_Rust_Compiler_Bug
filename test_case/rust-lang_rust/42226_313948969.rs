rust
struct Wrapper(String);
impl AsRef<str> for Wrapper {
   fn as_ref(&self) -> &str { &self.0 }
}

#[test]
fn test_me(){
   let foo_wrap = Wrapper(String::from("foo"));
   let bar = String::from("bar");
   // assert_ne!(foo_wrap.as_ref(), bar.as_ref()); // Doesn't compile
   assert_ne!(AsRef::<str>::as_ref(&foo_wrap), AsRef::<str>::as_ref(&bar)); // does work but is too verbose
}
