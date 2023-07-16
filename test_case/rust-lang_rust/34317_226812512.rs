 rust
macro_rules! method (
  ($name:ident<$a:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
      fn $name( $self_: $a, i: &[u8] ) -> ($a, $crate::IResult<&[u8], &[u8], u32>) { // instead of this
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
);
