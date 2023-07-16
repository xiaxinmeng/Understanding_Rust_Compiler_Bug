
error[E0277]: the trait bound `Point<f64, DimU2>: core::convert::From<Matrix<f64, DimU2, DimU1, ArrayStorage<f64, DimU2, DimU1>>>` is not satisfied
 --> main.rs:9:74
  |
9 |     let _: Point2<f64> = material_surface_element.map_reference_coords().into();
  |                                                                          ^^^^ the trait `core::convert::From<Matrix<f64, DimU2, DimU1, ArrayStorage<f64, DimU2, DimU1>>>` is not implemented for `Point<f64, DimU2>`
  |
  = help: the following implementations were found:
            <Point<N, D> as core::convert::From<Matrix<N, D, DimU1, <DefaultAllocator as Allocator<N, D>>::Buffer>>>
  = note: required because of the requirements on the impl of `core::convert::Into<Point<f64, DimU2>>` for `Matrix<f64, DimU2, DimU1, ArrayStorage<f64, DimU2, DimU1>>`

error: aborting due to previous error



nightly-2017-07-07 finished with exit code Some(101).
