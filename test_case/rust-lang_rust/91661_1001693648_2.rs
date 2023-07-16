
  412|       |impl<R: BlockRngCore + SeedableRng> SeedableRng for BlockRng64<R> {
  413|       |    type Seed = R::Seed;
  414|       |
  415|       |    #[inline(always)]
  416|      0|    fn from_seed(seed: Self::Seed) -> Self {
  417|      0|        Self::new(R::from_seed(seed))
  418|      0|    }
  419|       |
  420|       |    #[inline(always)]
  421|      0|    fn seed_from_u64(seed: u64) -> Self {
  422|      0|        Self::new(R::seed_from_u64(seed))
  423|      0|    }
  424|       |
  425|       |    #[inline(always)]
  426|      0|    fn from_rng<S: RngCore>(rng: S) -> Result<Self, Error> {
  427|      0|        Ok(Self::new(R::from_rng(rng)?))
  428|      0|    }
  429|       |}
