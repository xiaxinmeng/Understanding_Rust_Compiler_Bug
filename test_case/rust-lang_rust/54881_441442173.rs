rust
#[cfg_attr(all(),)] struct A; // trailing comma permitted as well as zero items.
#[cfg_attr(all(), must_use)] struct A;
#[cfg_attr(all(), must_use,)] struct A; // trailing permitted.
#[cfg_attr(foo, must_use, deprecated)] struct A;
#[cfg_attr(all(), must_use, deprecated,)] struct A; // trailing permitted.
