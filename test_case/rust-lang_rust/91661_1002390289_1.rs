
   45|       |impl Lcg128Xsl64 {
   46|       |    /// Multi-step advance functions (jump-ahead, jump-back)
   47|       |    ///
   48|       |    /// The method used here is based on Brown, "Random Number Generation
   49|       |    /// with Arbitrary Stride,", Transactions of the American Nuclear
   50|       |    /// Society (Nov. 1994).  The algorithm is very similar to fast
   51|       |    /// exponentiation.
   52|       |    ///
   53|       |    /// Even though delta is an unsigned integer, we can pass a
   54|       |    /// signed integer to go backwards, it just goes "the long way round".
   55|       |    ///
   56|       |    /// Using this function is equivalent to calling `next_64()` `delta`
   57|       |    /// number of times.
   58|       |    #[inline]
   59|      0|    pub fn advance(&mut self, delta: u128) {
   60|      0|        let mut acc_mult: u128 = 1;
   61|      0|        let mut acc_plus: u128 = 0;
   62|      0|        let mut cur_mult = MULTIPLIER;
   63|      0|        let mut cur_plus = self.increment;
   64|      0|        let mut mdelta = delta;
   65|       |
   66|      0|        while mdelta > 0 {
   67|      0|            if (mdelta & 1) != 0 {
   68|      0|                acc_mult = acc_mult.wrapping_mul(cur_mult);
   69|      0|                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
   70|      0|            }
   71|      0|            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
   72|      0|            cur_mult = cur_mult.wrapping_mul(cur_mult);
   73|      0|            mdelta /= 2;
   74|       |        }
   75|      0|        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
   76|      0|    }
   77|       |
   78|       |    /// Construct an instance compatible with PCG seed and stream.
   79|       |    ///
   80|       |    /// Note that the highest bit of the `stream` parameter is discarded
   81|       |    /// to simplify upholding internal invariants.
   82|       |    ///
   83|       |    /// Note that two generators with different stream parameters may be closely
   84|       |    /// correlated.
   85|       |    ///
   86|       |    /// PCG specifies the following default values for both parameters:
   87|       |    ///
   88|       |    /// - `state = 0xcafef00dd15ea5e5`
   89|       |    /// - `stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96`
   90|       |    pub fn new(state: u128, stream: u128) -> Self {
   91|       |        // The increment must be odd, hence we discard one bit:
   92|       |        let increment = (stream << 1) | 1;
   93|       |        Lcg128Xsl64::from_state_incr(state, increment)
   94|       |    }
   95|       |
   96|       |    #[inline]
   97|       |    fn from_state_incr(state: u128, increment: u128) -> Self {
   98|       |        let mut pcg = Lcg128Xsl64 { state, increment };
   99|       |        // Move away from initial value:
  100|       |        pcg.state = pcg.state.wrapping_add(pcg.increment);
  101|       |        pcg.step();
  102|       |        pcg
  103|       |    }
  104|       |
  105|       |    #[inline]
  106|       |    fn step(&mut self) {
  107|       |        // prepare the LCG for the next round
  108|       |        self.state = self
  109|       |            .state
  110|       |            .wrapping_mul(MULTIPLIER)
  111|       |            .wrapping_add(self.increment);
  112|       |    }
  113|       |}
  114|       |
  115|       |// Custom Debug implementation that does not expose the internal state
  116|       |impl fmt::Debug for Lcg128Xsl64 {
  117|       |    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  118|       |        write!(f, "Lcg128Xsl64 {{}}")
  119|       |    }
  120|       |}
  121|       |
  122|       |impl SeedableRng for Lcg128Xsl64 {
  123|       |    type Seed = [u8; 32];
  124|       |
  125|       |    /// We use a single 255-bit seed to initialise the state and select a stream.
  126|       |    /// One `seed` bit (lowest bit of `seed[8]`) is ignored.
  127|       |    fn from_seed(seed: Self::Seed) -> Self {
  128|       |        let mut seed_u64 = [0u64; 4];
  129|       |        le::read_u64_into(&seed, &mut seed_u64);
  130|       |        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
  131|       |        let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);
  132|       |
  133|       |        // The increment must be odd, hence we discard one bit:
  134|       |        Lcg128Xsl64::from_state_incr(state, incr | 1)
  135|       |    }
  136|       |}
