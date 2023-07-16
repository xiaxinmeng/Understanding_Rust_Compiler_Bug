
#[auto_derive]
trait foo {
 #[derive_scheme(EncodableScheme)]
 fn bar(x: uint); // Will just call bar() on all member data with the same parameter
 #[derive_scheme(CloneScheme)]
 fn baz() -> Self; // Will call baz() on all member data, and create a new object out of those
}
