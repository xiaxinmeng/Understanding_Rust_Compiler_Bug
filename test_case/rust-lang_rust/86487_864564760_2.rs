rust
  fn create_node<'fut>(&'fut self, node: &'fut DecodedNode)
    -> impl 'fut + Future<Output = Result<(),Error>> + Send
  {
    async move {
      let _captured = (self, node);
      if let Some((point,encoded)) = self.encode_node(node)? {
        let mut estore = self.estore.lock().await;
        let () = async {}.await; // to make `estore` cross an await point
        let res = dummy_of_typeof![ estore.create(point, encoded.into()).await ];
        let _: &dyn Send = &res;
        res?;
      }
    }
  }
