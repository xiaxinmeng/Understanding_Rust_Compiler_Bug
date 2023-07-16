diff
impl<'fixed> Bar<'fixed> {
-   fn hello (&self) {
// i.e.,
-   fn hello<'any> (self: &'any Bar<'fixed>) {
// to
+   fn hello<'any, 'any2> (this: &'any Bar<'any2>) {
// i.e.,
+   fn hello (this: &'_ Bar<'_>) {
// i.e.,
+   fn hello (this: &Bar<'_>) {
        println!("Hello {}", self.0);
    }
}
