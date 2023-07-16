rust
BBMatch {
  _D = &foo;
  _1 = GET_DISCRIMINANT(foo);
  SWITCH(_1, 0: BBarm)
}
...
BBArm {
  FakeUse(_D)
  // create bindings for arm here
  _x = &mut foo.downcast<Some>.0;
}
