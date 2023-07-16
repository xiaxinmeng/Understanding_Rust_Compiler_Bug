text
warning: diagnostic messages should start with lowercase letters
  |
1 | This associated function takes 0 generic arguments but 1 generic argument was supplied, which is illegal.
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file

warning: the word `illegal` is illegal
  |
1 | This associated function takes 0 generic arguments but 1 generic argument was supplied, which is illegal.
  |                                                                                                  ------- consider using more specific word, like `invalid`
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: illegals` to top of the test file

warning: diagnostic messages should not end with punctuations
  |
1 | This associated function takes 0 generic arguments but 1 generic argument was supplied, which is illegal.
  |                                                                                                         - this is a punctuation
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: punctuated-endings` to top of the test file
