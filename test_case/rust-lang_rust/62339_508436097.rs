rust
struct NoDerive(i32);

impl PartialEq for NoDerive {
    fn eq(&self, _: &Self) -> bool { false }
}

impl Eq for NoDerive { }

#[derive(PartialEq, Eq)]
struct WrapInline(NoDerive);

const WRAP_DIRECT_INLINE: WrapInline = WrapInline(NoDerive(0));

#[derive(PartialEq, Eq)]
struct WrapParam<T>(T);

const WRAP_DIRECT_PARAM: WrapParam<NoDerive> = WrapParam(NoDerive(0));
