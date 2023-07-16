
impl <'sess> Session <'sess> {
  fn subscription<'b>(&'b mut self) -> SubscriptionBuilder <'b, 'sess> {
    SubscriptionBuilder{
      session: self,
      frame: Frame,
      subscription: Subscription{subscription_name : "test"}
    }
  }
