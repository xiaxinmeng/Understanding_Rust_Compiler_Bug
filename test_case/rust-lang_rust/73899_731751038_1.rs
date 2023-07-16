
error: generic parameters may not be used in const operations
 --> src/main.rs:6:19
  |
6 |     Self:FooImpl<{N==0}>
  |                   ^ cannot perform const operation using `N`
  |
  = help: const parameters may only be used as standalone arguments, i.e. `N`
