
error[E0277]: the trait bound `Artichoke: Convert<i64, _>` is not satisfied
  --> src/main.rs:53:38
   |
53 |                     .map(|elem| self.convert(elem))
   |                                      ^^^^^^^ the trait `Convert<i64, _>` is not implemented for `Artichoke`
...
80 | array_to_ruby!(Int);
   | -------------------- in this macro invocation
   |
   = help: the following implementations were found:
             <Artichoke as Convert<std::vec::Vec<&'a str>, Value>>
             <Artichoke as Convert<std::vec::Vec<Value>, Value>>
             <Artichoke as Convert<std::vec::Vec<[type error]>, Value>>
             <Artichoke as Convert<std::vec::Vec<[type error]>, Value>>
           and 164 others
