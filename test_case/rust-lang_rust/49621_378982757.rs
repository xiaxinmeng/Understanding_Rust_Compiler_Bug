
PinBox<T>.pin(ptr) := exists |inner_ptr| ptr.points_to_owned(inner_ptr) && T.pin(inner_ptr)
