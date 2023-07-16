
   |
2  | fn foo() -> usize {
   | ----------------- fn() -> usize {foo} defined here
...
11 |     bar(foo);
   |         ^^^
   |         |
   |         expected usize, found fn item
   |         help: use parentheses to call this function: `foo()`
   |
   = note: expected type `usize`
              found type `fn() -> usize {foo}`
