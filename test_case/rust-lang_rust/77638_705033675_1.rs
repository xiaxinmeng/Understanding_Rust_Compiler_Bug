
error[E0277]: cannot multiply `&elliptic_curve::scalar::NonZeroScalar<C>` to `<C as ProjectiveArithmetic>::ProjectivePoint`
  --> ecdsa/src/sign.rs:90:58
   |
90 |             public_key: (C::ProjectivePoint::generator() * &self.secret_scalar).to_affine(),
   |                                                          ^ no implementation for `<C as ProjectiveArithmetic>::ProjectivePoint * &elliptic_curve::scalar::NonZeroScalar<C>`
   |
   = help: the trait `Mul<&elliptic_curve::scalar::NonZeroScalar<C>>` is not implemented for `<C as ProjectiveArithmetic>::ProjectivePoint`
