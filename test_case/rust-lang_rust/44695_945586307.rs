
error[E0369]: can't compare `&String` with `str`
 --> f16.rs:5:30
  |
5 |     let x: bool = string_ref == partial[..3];
  |                   ---------- ^^ ------------ str
  |                   |
  |                   &String
  |
help: `==` can be applied on `&str`
  |
5 |     let x: bool = string_ref == &partial[..3];
  |                                 +

error[E0369]: can't compare `&_` with `str`
 --> f16.rs:9:28
  |
9 |     let _ = owned.as_ref() == partial[..998];
  |             -------------- ^^ -------------- str
  |             |
  |             &_
  |
help: `==` can be applied on `&str`
  |
9 |     let _ = owned.as_ref() == &partial[..998];
  |                               +

error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   --> f16.rs:10:19
    |
10  |     let _ = owned.as_ref::<&str>() == &partial[..998]; // WTF is this thing with not resolving String: AsRef?
    |                   ^^^^^^-------- help: remove these generics
    |                   |
    |                   expected 0 generic arguments
    |
note: associated function defined here, with 0 generic parameters
   --> /local/home/ekuber/workspace/rust/library/core/src/convert/mod.rs:159:8
    |
159 |     fn as_ref(&self) -> &T;
    |        ^^^^^^

error[E0369]: can't compare `&&_` with `str`
  --> f16.rs:11:29
   |
11 |     let _ = &owned.as_ref() == partial[..998];
   |             --------------- ^^ -------------- str
   |             |
   |             &&_
