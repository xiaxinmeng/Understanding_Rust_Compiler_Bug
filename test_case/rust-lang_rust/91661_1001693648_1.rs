
  330|       |    /// Reset the number of available results.
  331|       |    /// This will force a new set of results to be generated on next use.
  332|       |    #[inline]
  333|      0|    pub fn reset(&mut self) {
  334|      0|        self.index = self.results.as_ref().len();
  335|      0|        self.half_used = false;
  336|      0|    }
