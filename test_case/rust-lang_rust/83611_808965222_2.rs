
[
  for<'a> ExistentialProjection::Trait(Self: Foo<'a>),
  for<'a> ExistentialProjection::Projection(<Self as Foo<'a>>::As = (),
  for<'b> ExistentialProjection::Trait(Self: Send)
]
