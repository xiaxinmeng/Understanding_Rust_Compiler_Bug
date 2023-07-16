
  std::unique_ptr<ArrayElems> &get_array_elems ()
  {
    rust_assert (internal_elements != nullptr);
    return internal_elements;
  }
  