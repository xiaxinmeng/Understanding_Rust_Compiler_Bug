rust
  use scaly::String;
  use std::ptr;
  String::create(ptr::null_mut(), ptr::null(), 0);
  // dereferences a null pointer
  // at scaly::containers::string::String::create()
  