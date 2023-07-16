
17 | impl<T> S<T> {
   |      - expected type parameter
...
22 |     fn foo3<U>(self, u: U) {
   |             - found type parameter
23 |         bar(self.t, u);
   |                     ^ expected type parameter `T`, found type parameter `U`
