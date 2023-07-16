
error: functions used as tests must have signature fn() -> ()
  --> src/lib.rs:18:1
   |
18 | / fn foo<T: Bar<Baz = String>>() -> <T as Bar>::Baz {
19 | |     let q = Quux{};
20 | |     <Quux as Bar>::rah(&q)
21 | | }
   | |_^
