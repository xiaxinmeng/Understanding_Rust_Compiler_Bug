
  // We can unroll by the upper bound amount if it's generally allowed or if
  // we know that the loop is executed either the upper bound or zero times.
  // (MaxOrZero unrolling keeps only the first loop test, so the number of
  // loop tests remains the same compared to the non-unrolled version, whereas
  // the generic upper bound unrolling keeps all but the last loop test so the
  // number of loop tests goes up which may end up being worse on targets with
  // constrained branch predictor resources so is controlled by an option.)
  // In addition we only unroll small upper bounds.
