diff
- let errors = iter::successors(Some(error), |error| error.source());
+ let errors = iter::successors(Some(error), |error| (*error).source());
