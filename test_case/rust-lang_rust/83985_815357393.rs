
  148|       |#[async_trait]
  149|       |trait AsyncTrait {
  150|       |    async fn covered(&self);
  151|       |    async fn uncovered(&self);
  152|       |}
  153|       |
  154|       |struct AsyncTraitImpl;
  155|       |
  156|       |#[async_trait]
  157|       |impl AsyncTrait for AsyncTraitImpl {
  158|      1|    async fn covered(&self) {
  159|      1|        println!("covered");
  160|      1|        async_func_from_trait_covered().await;
  161|      1|    }
  162|       |
  163|      0|    async fn uncovered(&self) {
  164|      0|        println!("uncovered");
  165|      0|    }
  166|       |}
  167|       |
