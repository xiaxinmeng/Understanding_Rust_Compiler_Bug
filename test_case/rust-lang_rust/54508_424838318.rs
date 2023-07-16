rust
impl<Ret, A, B, C> Eq for fn(A, B, C) -> Ret

impl<Ret, A, B, C> Eq for fn<'a>(&'a A, B, C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a>(&mut 'a A, B, C) -> Ret

impl<Ret, A, B, C> Eq for fn<'a, 'b>(&'a A, &'b B, C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b>(&mut 'a A, &'b B, C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b>(&mut 'a A, &mut 'b B, C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b>(&'a A, &mut 'b B, C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b>(A, &mut 'b B, C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b>(A, & 'b B, C) -> Ret

impl<Ret, A, B, C> Eq for fn<'a, 'b, 'c>(&'a A, &'b B, &'c C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b, 'c>(&mut 'a A, &'b B, &'c C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b, 'c>(&mut 'a A, &mut 'b B, &mut 'c C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b, 'c>(&mut 'a A, &'b B, &mut 'c C) -> Ret
impl<Ret, A, B, C> Eq for fn<'a, 'b, 'c>(&'a A, &'b B, &mut 'c C) -> Ret

// ... and so on.
