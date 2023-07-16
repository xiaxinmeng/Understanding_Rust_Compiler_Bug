rust
enum E { V }

trait T {
    type V;
    const C: Self::V;
}

impl T for E {
    type V = u8;
    const C: Self::V = 0;
          // ------- currently refers to `type V`
          //         with RFC 2593 it refers to the *type* `E::V`.
}
