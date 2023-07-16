
/vol/gcc/src/git/rust/gcc/rust/backend/rust-compile-intrinsic.cc: In function 'tree_node* Rust::Compile::transmute_intrinsic_handler(Context*, Rust::TyTy::BaseType*)':
/vol/gcc/src/git/rust/gcc/rust/backend/rust-compile-intrinsic.cc:527:69: error: format '%lu' expects argument of type 'long unsigned int', but argument 4 has type 'long long unsigned int' [-Werror=format=]
  527 |       rust_inform (fntype->get_ident ().locus, "source type: %qs (%lu bits)",
      |                                                                   ~~^
      |                                                                     |
      |                                                                     long unsigned int
      |                                                                   %llu
  528 |                    fntype->get_params ().at (0).second->as_string ().c_str (),
  529 |                    source_size);
      |                    ~~~~~~~~~~~
      |                    |
      |                    long long unsigned int
/vol/gcc/src/git/rust/gcc/rust/backend/rust-compile-intrinsic.cc:530:69: error: format '%lu' expects argument of type 'long unsigned int', but argument 4 has type 'long long unsigned int' [-Werror=format=]
  530 |       rust_inform (fntype->get_ident ().locus, "target type: %qs (%lu bits)",
      |                                                                   ~~^
      |                                                                     |
      |                                                                     long unsigned int
      |                                                                   %llu
  531 |                    fntype->get_return_type ()->as_string ().c_str (),
  532 |                    target_size);
      |                    ~~~~~~~~~~~
      |                    |
      |                    long long unsigned int
