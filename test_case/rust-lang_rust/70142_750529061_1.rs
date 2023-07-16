rust
/// Wrapper around `failure::Error` with support for additional json context
#[derive(Deref, DerefMut)]
pub struct InternalError {
	inner: Box<InternalErrorInner>,
}

pub struct InternalErrorInner {
	pub err: failure::Error,
	pub context: BTreeMap<String, serde_json::Value>,
	#[cfg(feature = "sentry")]
	pub patch_event: Option<Box<dyn Fn(&mut sentry_helpers::sentry::protocol::Event) + Send + Sync>>,
	_private: (),
}
