rust
pub fn provide(providers: &mut Providers) {
    *providers = Providers { visibility, ..*providers };
}
