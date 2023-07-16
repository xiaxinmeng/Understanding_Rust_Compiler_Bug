
This function will return an instance of [`Error`] on error.

Though it is possible for implementors of this trait to return an error, at the time of writing these docs, no implementation of [`std::fmt::Write`] in the standard library returns such an error.

When working with external crates, it is advisable to check the implementation of this trait and program around any possible [`Error`]s.
