
impl Both for @Foo {
    fn foo(&self) { self.foo(); }
    fn bar(self) { self.bar(); } // illegal: cannot invoke by-val-self method on object
}
