rust
error[E0277]: the trait bound `Bar: Deserialize<'_>` is not satisfied
    --> src/main.rs:123:37
     |
123  |                         match match _serde::de::SeqAccess::next_element::<Bar>(&mut __seq) {
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Deserialize<'_>` is not implemented for `Bar`
     |
note: required by `next_element`
    --> /home/joonas/.cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.130/src/de/mod.rs:1703:5
     |
1703 | /     fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
1704 | |     where
1705 | |         T: Deserialize<'de>,
     | |____________________________^

