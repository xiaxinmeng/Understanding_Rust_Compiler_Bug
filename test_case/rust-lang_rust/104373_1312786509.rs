
error[[E0063]](https://doc.rust-lang.org/stable/error-index.html#E0063): Missing comma: to set the remaining fields of this `Outer` object using `Default::default` add a comma after the last named field before the `..`.
  --> src/main.rs:14:5
   |
14 |     Outer {
   |
   |
18 |         }
   |     ^^^ missing comma after named item
   |          ..Default::default()
   |       }
   |          +
