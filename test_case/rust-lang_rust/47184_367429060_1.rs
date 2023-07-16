
/// Encodes a user's type assertion. These need to be preserved
/// intact so that NLL can respect them. For example:
///
///     let (a, b): (T, U) = y;
///
/// Here, we would insert a `UserAssertTy<(T, U)>(y)` instruction to check
/// that the type of `y` is the right thing.
UserAssertTy(Ty<'tcx>, Local),
