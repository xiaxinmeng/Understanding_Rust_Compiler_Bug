
trait Incrementable {
 #[derive_scheme(CloneScheme)]
 fn increment(&self) -> Self; // When incrementing, all member data is incremented, and an object is made from that data
}
