rust
impl From<Cow<str>> for Box<str> {...}
impl From<Cow<CStr>> for Box<CStr> {...}
impl From<Cow<OsStr>> for Box<OsStr> {...}
impl From<Cow<Path>> for Box<Path> {...}

impl<T> From<Cow<[T]>> for Box<[T]>
where
    T: Copy,
{...}

impl<'a, B> From<Cow<'a, B>> for Rc<B>
where
    B: ToOwned + ?Sized,
    Rc<B>: From<&'a B> + From<B::Owned>,
{...}

impl<'a, B> From<Cow<'a, B>> for Arc<B>
where
    B: ToOwned + ?Sized,
    Arc<B>: From<&'a B> + From<B::Owned>,
{...}
