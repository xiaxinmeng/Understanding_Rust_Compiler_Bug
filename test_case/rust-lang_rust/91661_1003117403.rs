
   54|       |    /// signed integer to go backwards, it just goes "the long way round".
   55|       |    ///
   56|       |    /// Using this function is equivalent to calling `next_64()` `delta`
   57|       |    /// number of times.
   58|       |    #[inline]
   59|     20|    pub fn advance(&mut self, delta: u128) {
   60|     20|        let mut acc_mult: u128 = 1;
   61|     20|        let mut acc_plus: u128 = 0;
   62|     20|        let mut cur_mult = MULTIPLIER;
   63|     20|        let mut cur_plus = self.increment;
   64|     20|        let mut mdelta = delta;
   65|       |
   66|    120|        while mdelta > 0 {
   67|    100|            if (mdelta & 1) != 0 {
   68|     40|                acc_mult = acc_mult.wrapping_mul(cur_mult);
   69|     40|                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
   70|     60|            }
   71|    100|            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
   72|    100|            cur_mult = cur_mult.wrapping_mul(cur_mult);
   73|    100|            mdelta /= 2;
   74|       |        }
   75|     20|        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
   76|     20|    }
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
   90|      1|    pub fn new(state: u128, stream: u128) -> Self {
   91|      1|        // The increment must be odd, hence we discard one bit:
   92|      1|        let increment = (stream << 1) | 1;
   93|      1|        Lcg128Xsl64::from_state_incr(state, increment)
   94|      1|    }
   95|       |
   96|       |    #[inline]
   97|     25|    fn from_state_incr(state: u128, increment: u128) -> Self {
   98|     25|        let mut pcg = Lcg128Xsl64 { state, increment };
   99|     25|        // Move away from initial value:
  100|     25|        pcg.state = pcg.state.wrapping_add(pcg.increment);
  101|     25|        pcg.step();
  102|     25|        pcg
  103|     25|    }
  104|       |
  105|       |    #[inline]
  106|    439|    fn step(&mut self) {
  107|    439|        // prepare the LCG for the next round
  108|    439|        self.state = self
  109|    439|            .state
  110|    439|            .wrapping_mul(MULTIPLIER)
  111|    439|            .wrapping_add(self.increment);
  112|    439|    }
  113|       |}
