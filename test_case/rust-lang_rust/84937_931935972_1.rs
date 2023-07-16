
51 |     map_project_broken(bar, |bar, _| bar.string_1)
   |     ^^^^^^^^^^^^^^^^^^      --------------------- found signature of `for<'r> fn(Bar<'r>, PhantomData<&'r ()>) -> _`
   |     |
   |     expected signature of `for<'a> fn(<Bar<'_> as MiniYokeable<'a>>::Output, PhantomData<&'a ()>) -> _`
