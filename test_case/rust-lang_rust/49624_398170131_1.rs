
forall<A...> { // parameters from impl A
  forall<B..> { // parameters from impl B
    if (both impls apply) {
      forall<C..> { // parameters from impl C
        impl C applies
      }
    }
  }
}
