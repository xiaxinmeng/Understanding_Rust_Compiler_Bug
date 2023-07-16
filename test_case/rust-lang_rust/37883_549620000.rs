
error[E0277]: cannot multiply `<E as EuclideanSpace>::Real` to `<E as EuclideanSpace>::Coordinates`
  --> src/main.rs:25:26
   |
24 |     fn powf(&self, n: <E::Coordinates as Module>::Ring) -> E::Coordinates {
   |                                                                          - help: consider further restricting the associated type: `where <E as EuclideanSpace>::Coordinates: std::ops::Mul<<E as EuclideanSpace>::Real>`
25 |         self.to_vector() * n
   |                          ^ no implementation for `<E as EuclideanSpace>::Coordinates * <E as EuclideanSpace>::Real`
   |
   = help: the trait `std::ops::Mul<<E as EuclideanSpace>::Real>` is not implemented for `<E as EuclideanSpace>::Coordinates`

error[E0308]: mismatched types
  --> src/main.rs:25:28
   |
25 |         self.to_vector() * n
   |                            ^ expected Module::Ring, found EuclideanSpace::Real
   |
   = note: expected type `<<E as EuclideanSpace>::Coordinates as Module>::Ring`
              found type `<E as EuclideanSpace>::Real`
