
Stablize slice::strip_prefix and strip_suffix, with SlicePattern

We hope later to extend `core::str::Pattern` to slices too, perhaps as
part of stabilising that.  We want to minimise the amount of type
inference breakage when we do that, so we don't want to stabilise
strip_prefix and strip_suffix taking a simple `&[T]`.

@KodrAus suggested the approach of introducing a new perma-unstable
trait, which reduces this future inference break risk.

I found it necessary to make two impls of this trait, as the unsize
coercion don't apply when hunting for trait implementations.

Since SlicePattern's only method returns a reference, and the whole
trait is just a wrapper for slices, I made the trait type be the
non-reference type [T] or [T;N] rather than the reference.  Otherwise
the trait would have a lifetime parameter.

I marked both the no-op conversion functions `#[inline]`.  I'm not
sure if that is necessary but it seemed at the very least harmless.
