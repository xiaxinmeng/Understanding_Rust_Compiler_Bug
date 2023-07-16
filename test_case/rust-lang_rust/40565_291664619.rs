rust
error[E0277]: the trait bound `{integer}: std::ops::Add<std::option::Option<{integer}>>` is not satisfied
 --> <anon>:3:5
  |
3 |     1 + Some(1);
  |     ^^^^^^^^^^^ the trait `std::ops::Add<std::option::Option<{integer}>>` is not implemented for `{integer}`
  |
  = help: the following implementations were found:
            <std::borrow::Cow<'a, str> as std::ops::Add<&'a str>>
            <std::borrow::Cow<'a, str> as std::ops::Add>
            <u32 as std::ops::Add>
            <&'a u32 as std::ops::Add<u32>>
          and 106 others

error[E0277]: the trait bound `usize: std::ops::Sub<std::option::Option<{integer}>>` is not satisfied
 --> <anon>:4:5
  |
4 |     1 as usize - Some(1);
  |     ^^^^^^^^^^^^^^^^^^^^ the trait `std::ops::Sub<std::option::Option<{integer}>>` is not implemented for `usize`
  |
  = help: the following implementations were found:
            <usize as std::ops::Sub>
            <&'a usize as std::ops::Sub<usize>>
            <usize as std::ops::Sub<&'a usize>>
            <&'b usize as std::ops::Sub<&'a usize>>

error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<std::result::Result<{integer}, _>>` is not satisfied
 --> <anon>:5:5
  |
5 |     1 == Ok(1);
  |     ^^^^^^^^^^ the trait `std::cmp::PartialEq<std::result::Result<{integer}, _>>` is not implemented for `{integer}`
  |
  = help: the following implementations were found:
            <&'a A as std::cmp::PartialEq<&'b B>>
            <&'a mut A as std::cmp::PartialEq<&'b mut B>>
            <&'a A as std::cmp::PartialEq<&'b mut B>>
            <&'a mut A as std::cmp::PartialEq<&'b B>>
          and 697 others
