
error[E0271]: type mismatch resolving `<MyStruct<&str> as MyTrait>::Item == i32`
  --> src/main.rs:16:5
   |
13 | fn func<T: MyTrait<Item=i32>>(t: T) { }
   |    ----            -------- required by this bound in `func`
...
16 |     func(MyStruct { item: "str" });
   |     ^^^^ expected &str, found i32
   |
   = note: expected type `&str`
              found type `i32`
