
fn mode_2(_b: Foo<'_>) -> impl Future<Output=()> + '_ { // note the relaxing of the lifetime constraint to the return type
  async {}
}
