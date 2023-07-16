 rust
enum FindCapturesInner<'r, 't> {
    Dynamic(re_trait::FindCaptures<'t, ExecNoSyncStr<'r>>),
    Plugin(re_trait::FindCaptures<'t, Plugin>),
}
