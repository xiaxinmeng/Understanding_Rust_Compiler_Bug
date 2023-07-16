 diff
#[stable(feature = "rust1", since = "1.0.0")]
-pub struct Union<'a>(TwoBitPositions<'a>);
+pub struct Union<'a>(BlockIter<TwoBitPositions<'a>>);
