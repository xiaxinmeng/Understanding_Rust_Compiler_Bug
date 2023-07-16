
error[[E0277]](https://doc.rust-lang.org/stable/error_codes/E0277.html): the trait bound `Bar: Deserialize<'_>` is not satisfied
    --> src/main.rs:8:10
     |
8    |     foo: Bar,
     |          ^^^ the trait `Deserialize<'_>` is not implemented for `Bar`
     |
     = help: the following other types implement trait `Deserialize<'de>`:
               &'a Path
               &'a [u8]
               &'a str
               ()
               (T0, T1)
               (T0, T1, T2)
               (T0, T1, T2, T3)
               (T0, T1, T2, T3, T4)
             and 130 others
note: required by a bound in `next_element`
    --> /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.156/src/de/mod.rs:1729:12
     |
1729 |         T: Deserialize<'de>,
     |            ^^^^^^^^^^^^^^^^ required by this bound in `SeqAccess::next_element`
...
