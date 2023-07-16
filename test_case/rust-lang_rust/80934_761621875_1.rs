
error[E0282]: type annotations needed
  --> estebank_counter.rs:18:1
   |
18 | / fn foo<T: Bar<Baz = String>>() -> <T as Bar>::Baz {
19 | |     let q = Quux{};
20 | |     <Quux as Bar>::rah(&q)
21 | | }
   | |_^ cannot infer type for type parameter `T` declared on the function `assert_test_result`
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)
