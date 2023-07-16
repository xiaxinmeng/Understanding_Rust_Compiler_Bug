 rust
#![crate_type = "lib"]

trait RegularExpression: Sized {
    type Text: ?Sized;
}

struct ExecNoSyncStr<'a>(&'a u8);

impl<'c> RegularExpression for ExecNoSyncStr<'c> {
    type Text = str;
}

struct FindCaptures<'t, R>(&'t R::Text) where R: RegularExpression, R::Text: 't;

enum FindCapturesInner<'r, 't> {
    Dynamic(FindCaptures<'t, ExecNoSyncStr<'r>>),
}
