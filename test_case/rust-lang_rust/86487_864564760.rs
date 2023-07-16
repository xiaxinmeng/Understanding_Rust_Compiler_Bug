rust
  /// First, unsugar the `async fn` into an `fn … -> impl Future…`:
  fn create_node<'slf, 'node, 'fut>(self: &'slf Self, node: &'node DecodedNode)
    -> impl 'fut + Future<Output = Result<(),Error>> + Send
  where
    &'slf Self : 'fut, // 'slf : 'fut
    &'node DecodedNode : 'fut, // 'node : 'fut
  {
    async move { let _captures_args = (&self, &node);
      if let Some((point,encoded)) = self.encode_node(node)? {
        let mut estore = self.estore.lock().await;
        let () = async {}.await; // to make `estore` cross an await point

        // with the following trick, we get a value of the right type (`Result…`),
        // but without the intermediate future polluting the state machine.
        let res = /* estore.create(point, encoded.into()).await */ {
          let mut it = None.unwrap();
          if false { // type-inference arm.
            let _ = async {
              it = estore.create(point, encoded.into()).await;
            };
            loop {}
          }
          it
        };
        // All these checks pass.
        let _: &(dyn Send + 'static) = &res;
        if let Err(err) = res {
          let err = Error::from(err);
          let _: &(dyn Send + 'static) = &err;
          return Err(err);
        }
      }
      Ok(())
    }
  }
