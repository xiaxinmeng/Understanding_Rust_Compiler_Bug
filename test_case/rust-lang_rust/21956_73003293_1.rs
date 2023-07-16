 rust
use std::collections::hash_map::HashMap;

pub struct Frame;

pub struct Subscription<'a>{
  subscription_name: &'a str
}

pub struct Session <'a> {
  subscriptions: HashMap<String, Subscription <'a>>
}

pub struct SubscriptionBuilder <'a, 'sess: 'a> {
  pub session: &'a mut Session<'sess>,
  pub frame: Frame,
  pub subscription: Subscription<'a>
}

impl <'sess> Session <'sess> {
  fn subscription<'b>(&'b mut self) -> SubscriptionBuilder <'b, 'sess> {
    SubscriptionBuilder{
      session: self,
      frame: Frame,
      subscription: Subscription{subscription_name : "test"}
    }
  }

  fn do_nothing(&self){
    // Do nothing.
  }
}

impl <'a, 'sess> SubscriptionBuilder <'a, 'sess> {
  pub fn create(self) -> String {
    "subscription_id".to_string()
  }
}

fn main() {
    let mut session = Session{subscriptions : HashMap::new()};
    let sub_id = session.subscription().create();
    session.do_nothing();
}
