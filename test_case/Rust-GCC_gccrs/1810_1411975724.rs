rust
#[lang = "clone"]
trait Copy{
		fn clone(&self) -> Self;

		fn clone_from(& mut self, source: &Self)
}
