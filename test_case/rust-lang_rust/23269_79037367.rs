
<anon>:14         "foo".split("o").rev().collect::<Vec<_>>()
                                         ^~~~~~~~~~~~~~~~~~~
error: type `core::iter::Rev<core::str::Split<'_, &str>>` does not implement any method in scope named `collect`
note: Possible trait that provides `collect`: `core::iter::Iterator`.
note: `core::iter::Rev<core::str::Split<'_, &str>>` would implement `core::iter::Iterator` if
      `core::str::Split<'_, &str>` would implement `core::iter::DoubleEndedIterator`
note: `core::str::Split<'_, &str>` would implement `core::iter::DoubleEndedIterator` if
      `<&str as Pattern<'_>>::Searcher` would implement `core::str::DoubleEndedSearcher`
note: `<&str as Pattern<'_>>::Searcher` does not implement `core::str::DoubleEndedSearcher`
