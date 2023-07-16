
error[E0277]: the trait bound `Vec<u8>: Extend<{float}>` is not satisfied
 --> src/main.rs:4:34
  |
4 |     once(async { 0.0 }).collect().await
  |                                  ^^^^^^ the trait `Extend<{float}>` is not implemented for `Vec<u8>`
  |
  = help: the following other types implement trait `Extend<A>`:
            <Vec<T, A> as Extend<&'a T>>
            <Vec<T, A> as Extend<T>>
  = note: required for `Collect<futures::stream::Once<[async block@src/main.rs:4:10: 4:23]>, Vec<u8>>` to implement `futures::Future`

error[E0277]: the trait bound `Vec<u8>: Extend<{float}>` is not satisfied
   --> src/main.rs:4:25
    |
4   |     once(async { 0.0 }).collect().await
    |                         ^^^^^^^ the trait `Extend<{float}>` is not implemented for `Vec<u8>`
    |
    = help: the following other types implement trait `Extend<A>`:
              <Vec<T, A> as Extend<&'a T>>
              <Vec<T, A> as Extend<T>>
note: required by a bound in `futures::StreamExt::collect`
   --> /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.25/src/stream/stream/mod.rs:516:29
    |
516 |     fn collect<C: Default + Extend<Self::Item>>(self) -> Collect<Self, C>
    |                             ^^^^^^^^^^^^^^^^^^ required by this bound in `StreamExt::collect`
