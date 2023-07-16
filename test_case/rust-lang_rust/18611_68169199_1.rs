 rust
pub trait PbMsg {
  type Obj;
}

// ....

impl PushMsg for PbMsg {
  type Obj = super::objects::Push;
}
// ...and a lot of very similar definitions
