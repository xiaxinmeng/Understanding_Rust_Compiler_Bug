
> //! Unlike native toolchains, rustc only currently adds the libc's objects during linking,
> //! but not gcc's. As a result rustc cannot link with C++ static libraries (#36710)
> //! when linking in self-contained mode.
> 