
  226|       |
  227|       |    #[test]
  228|      1|    fn cover_async_trait() {
  ------------------
  | codecovsample::tests::cover_async_trait::{closure#0}:
  |  228|      1|    fn cover_async_trait() {
  ------------------
  229|      1|        block_on(async {
  230|      1|            let x: Box<dyn AsyncTrait> = Box::new(AsyncTraitImpl);
  231|      1|            x.covered().await;
  232|      1|        });
  233|      1|    }
  ------------------
  | codecovsample::tests::cover_async_trait:
  |  228|      1|    fn cover_async_trait() {
  |  229|      1|        block_on(async {
  |  230|       |            let x: Box<dyn AsyncTrait> = Box::new(AsyncTraitImpl);
  |  231|       |            x.covered().await;
  |  232|      1|        });
  |  233|      1|    }
  ------------------
  234|       |}
