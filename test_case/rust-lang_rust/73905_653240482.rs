rust
pub trait Component
where
  Self: Sized + 'static,
  Self::ModelMsg: Clone,
  Self::ViewMsg: Clone,
  Self::DomNode: JsCast + AsRef<Node> + Clone,
