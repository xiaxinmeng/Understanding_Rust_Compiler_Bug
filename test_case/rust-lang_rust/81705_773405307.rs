rust
std::iter::successors::<&(dyn Error), _>(Some(&my_error), |e| e.source())
