` in doctests](https://github.com/rust-lang/rust/pull/96573#issuecomment-1115248925), but fundamentally I agree that making the situation worse is not great (I'm working to make it better).

This PR is a *mechanism* for signalling an ignore during runtime, but I suspect that the actual API might look more like `fn test::ignore(reason: Option<String>)`, which would allow the test runtime to check a global to see if `--ignored` was used, and ignore the ignoring. It might also be useful to have a `test::skip(reason: String) -> !` that is guaranteed to abort the test, but users could also "just" write `test::ignore(None); panic("I meant it");` if letting the test fail normally is a bad idea.

> requirement for `#![feature(test)]`

I mean, initially, it's going to be feature gated anyway, so while this is something that needs to be resolved for stabilization, it doesn't need to block implementation.

I don't see a way to properly expose this feature without stabilizing (minimally the existence of) the test support crate, though. You *could* provide the APIs as part of `std`... but that seriously smells like putting items in the wrong place just to get around the test crate being unstable currently.